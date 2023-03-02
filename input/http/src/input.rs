use std::{net::SocketAddr, sync::Arc};

use hyper::{Body, Request, Response};
use tokio::net::{TcpListener, TcpSocket};

use crate::{run_high_http, Config, Result, TlsConfig};

pub struct HttpInput<Graph> {
    graph: Graph,
    tcp_listener: TcpListener,

    #[cfg(feature = "openssl-tls")]
    ssl_acceptor: Option<openssl::ssl::SslAcceptor>,

    #[cfg(feature = "rustls-tls")]
    ssl_acceptor: Option<tokio_rustls::TlsAcceptor>,
}

impl<Graph> HttpInput<Graph> {
    fn new_tcp_listener(la: SocketAddr, _ld: Option<String>) -> Result<TcpListener> {
        let listener = match &la {
            SocketAddr::V4(_) => {
                let socket = TcpSocket::new_v4()?;
                socket.bind(la)?;
                socket
            }
            SocketAddr::V6(_) => {
                let socket = TcpSocket::new_v6()?;
                socket.bind(la)?;
                socket
            }
        };

        #[cfg(target_os = "linux")]
        if let Some(device) = &_ld {
            listener.bind_device(Some(device.as_bytes()))?;
        }

        listener.set_reuseport(true)?;
        listener.set_reuseaddr(true)?;

        Ok(listener.listen(1024)?)
    }

    #[cfg(feature = "openssl-tls")]
    fn new_tls_listener(tls: Vec<TlsConfig>, alpn: Vec<u8>) -> Result<openssl::ssl::SslAcceptor> {
        use openssl::ssl::{
            select_next_proto, AlpnError, NameType, SniError, SslAcceptor, SslMethod,
        };

        let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls_server())?;
        let configs = Arc::new(crate::utils::openssl_utils::build_ssl_context_map(tls)?);

        acceptor.set_servername_callback(move |ssl, _| -> std::result::Result<(), SniError> {
            let domain = ssl.servername(NameType::HOST_NAME);

            if let Some(domain) = domain {
                if let Some(ctx) = configs.get(domain) {
                    log::info!("Match sni: {domain}");
                    match ssl.set_ssl_context(ctx) {
                        Ok(()) => Ok(()),
                        Err(e) => {
                            log::error!("ssl_context set error: {e}");
                            Err(SniError::ALERT_FATAL)
                        }
                    }
                } else {
                    Err(SniError::ALERT_FATAL)
                }
            } else {
                Err(SniError::ALERT_FATAL)
            }
        });

        // TODO: This is a bug, seem caused by openssl
        acceptor.set_alpn_select_callback(move |_, a| {
            log::info!("Alpn is: {a:?}");
            select_next_proto(&alpn, a).ok_or(AlpnError::NOACK)
        });

        Ok(acceptor.build())
    }

    #[cfg(feature = "rustls-tls")]
    fn new_tls_listener(
        tlses: Vec<TlsConfig>,
        http2: bool,
        http1: bool,
    ) -> Result<tokio_rustls::TlsAcceptor> {
        use std::io::Cursor;

        use rustls::{
            server::ResolvesServerCertUsingSni,
            sign::{any_supported_type, CertifiedKey},
            Certificate, PrivateKey, ServerConfig,
        };
        use rustls_pemfile::{read_one, Item};
        use tokio_rustls::TlsAcceptor;

        use crate::{utils, Error};

        let mut cert_resolver = ResolvesServerCertUsingSni::new();

        for tls in tlses {
            let mut pk = Cursor::new(&tls.private_key);
            let item = read_one(&mut pk)
                .expect("Lower error")
                .ok_or(Error::ParsePEMFaile)?;
            let key = match item {
                Item::ECKey(k) => any_supported_type(&PrivateKey(k))?,
                Item::RSAKey(k) => any_supported_type(&PrivateKey(k))?,
                Item::PKCS8Key(k) => any_supported_type(&PrivateKey(k))?,
                _ => return Err(Error::ParsePEMFaile),
            };

            let mut certs = Vec::with_capacity(tls.certificate.len());

            for cert in tls.certificate {
                let mut cert = Cursor::new(cert);
                let item = read_one(&mut cert)
                    .expect("Lower error")
                    .ok_or(Error::ParsePEMFaile)?;

                let cert = match item {
                    Item::X509Certificate(k) => k,
                    _ => return Err(Error::ParsePEMFaile),
                };

                certs.push(Certificate(cert));
            }

            let ck = CertifiedKey::new(certs, key);

            println!("asdsdsadsdsadasdas");
            cert_resolver.add(&tls.sni, ck)?;
            println!("asdsdsadsdsadasdas");
        }

        let mut builder = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_cert_resolver(Arc::new(cert_resolver));

        let mut alpns = Vec::with_capacity(2);

        if http2 {
            alpns.push(utils::http2_alpn())
        }

        if http1 {
            alpns.push(utils::http1_alpn());
        }

        builder.alpn_protocols = alpns;

        Ok(TlsAcceptor::from(Arc::new(builder)))
    }

    pub fn new(graph: Graph, config: Config) -> Result<Self> {
        #[cfg(any(feature = "openssl-tls", feature = "rustls-tls"))]
        let ssl_acceptor = if config.tls.is_empty() {
            None
        } else {
            Some(Self::new_tls_listener(
                config.tls,
                config.http2,
                config.http1,
            )?)
        };

        let tcp_listener = Self::new_tcp_listener(config.listen_addr, config.listen_device)?;

        Ok(Self {
            graph,
            #[cfg(any(feature = "openssl-tls", feature = "rustls-tls"))]
            ssl_acceptor,
            tcp_listener,
        })
    }
}

impl<Graph> HttpInput<Graph>
where
    Graph: Sync + Send + 'static,
{
    pub async fn run(self) -> Result<()> {
        let graph = Arc::new(self.graph);

        if cfg!(any(feature = "openssl-tls", feature = "rustls-tls")) {
            if let Some(tls) = self.ssl_acceptor {
                #[cfg(feature = "rustls-tls")]
                if let Err(e) = accept_https(self.tcp_listener, tls, graph).await {
                    log::error!("Accept error: {e:?}");
                }
            } else {
                run_high_http(self.tcp_listener, graph).await?;
            }
        } else {
            run_high_http(self.tcp_listener, graph).await?;
        }

        Ok(())
    }
}

#[cfg(feature = "rustls-tls")]
pub(crate) async fn accept_https<G>(
    tcp: TcpListener,
    tls: tokio_rustls::TlsAcceptor,
    graph: Arc<G>,
) -> Result<()>
where
    G: Sync + Send + 'static,
{
    use std::pin::Pin;

    use hyper::{server::conn::Http, service::service_fn};
    use tokio::net::TcpStream;
    use tokio_rustls::Accept;

    async fn _handle<G>(stream: Accept<TcpStream>, graph: Arc<G>) -> Result<()>
    where
        G: Send + Sync + 'static,
    {
        let stream = stream.await?;

        let sf = service_fn(|req| {
            let graph = graph.clone();

            handle_http(req, graph)
        });

        let mut conn = Http::new().serve_connection(stream, sf);

        Pin::new(&mut conn).graceful_shutdown();

        conn.await?;
        Ok(())
    }

    loop {
        let (stream, _) = tcp.accept().await?;

        let stream = tls.accept(stream);

        let graph = graph.clone();

        tokio::spawn(async move {
            if let Err(e) = _handle(stream, graph).await {
                log::error!("Handle stream error: {e:?}");
            }
        });
    }
}

// #[cfg(feature = "openssl-tls")]
// pub(crate) async fn accept_https(tcp: TcpStream, ssl: openssl::ssl::Ssl) -> Result<()> {
//     use std::pin::Pin;
//
//     use hyper::server::conn::Http;
//     use tokio_openssl::SslStream;
//
//     let mut stream = SslStream::new(ssl, tcp)?;
//
//     Pin::new(&mut stream).accept().await?;
//
//     let mut conn = Http::new().serve_connection(stream, service_fn(handle_http));
//
//     Pin::new(&mut conn).graceful_shutdown();
//
//     conn.await?;
//
//     Ok(())
// }

pub(crate) async fn handle_http<G>(_req: Request<Body>, graph: Arc<G>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello !")))
}
