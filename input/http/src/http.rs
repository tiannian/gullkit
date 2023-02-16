use std::{net::SocketAddr, result::Result as StdResult, sync::Arc};

use hyper::{
    server::conn::AddrIncoming,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use openssl::ssl::{select_next_proto, AlpnError, NameType, SniError, Ssl, SslAcceptor, SslMethod};
use tokio::net::{TcpListener, TcpSocket};

use crate::{accept_https, utils, Config, Error, Result, TlsConfig};

pub struct HttpInput<Graph> {
    graph: Graph,
    ssl_acceptor: Option<SslAcceptor>,
    tcp_listener: TcpListener,
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
        // listener.set_reuseaddr(true)?;

        Ok(listener.listen(1024)?)
    }

    pub fn new_tls_listener(tls: Vec<TlsConfig>, alpn: Vec<u8>) -> Result<SslAcceptor> {
        let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls_server())?;
        let configs = Arc::new(utils::build_ssl_context_map(tls)?);

        acceptor.set_servername_callback(move |ssl, _| -> StdResult<(), SniError> {
            let domain = ssl.servername(NameType::HOST_NAME);

            // log::info!("Receive sni: {domain:?}");
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

        acceptor.set_alpn_protos(&alpn)?;

        acceptor.set_alpn_select_callback(move |_, a| {
            log::info!("Alpn is: {a:?}");
            select_next_proto(&alpn, a).ok_or(AlpnError::NOACK)
        });

        Ok(acceptor.build())
    }

    pub fn new(graph: Graph, config: Config) -> Result<Self> {
        let alpn = config.alpn();

        let tcp_listener = Self::new_tcp_listener(config.listen_addr, config.listen_device)?;
        let ssl_acceptor = if config.tls.is_empty() {
            None
        } else {
            Some(Self::new_tls_listener(config.tls, alpn)?)
        };

        Ok(Self {
            graph,
            ssl_acceptor,
            tcp_listener,
        })
    }
}

impl<Graph> HttpInput<Graph> {
    pub async fn run(self) -> Result<()> {
        let _graph = Arc::new(self.graph);

        if let Some(ssl_acceptor) = self.ssl_acceptor {
            loop {
                let (stream, addr) = self.tcp_listener.accept().await?;
                log::debug!("Https connection from: {addr}");

                let mut ssl = Ssl::new(ssl_acceptor.context())?;

                let mut alpn = Vec::new();
                alpn.push(2);
                alpn.extend_from_slice(b"h2");
                ssl.set_alpn_protos(&alpn)?;

                tokio::spawn(async {
                    if let Err(e) = accept_https(stream, ssl).await {
                        log::error!("Accept https error: {e:?}");
                    }
                });
            }
        } else {
            let acceptor = AddrIncoming::from_listener(self.tcp_listener)?;

            let builder = Server::builder(acceptor);

            let mksvc = make_service_fn(|_| async {
                let sf = service_fn(|req| async { handle_http(req).await });

                Ok::<_, Error>(sf)
            });

            builder.serve(mksvc).await?;

            Ok(())
        }
    }
}

pub(crate) async fn handle_http(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello !")))
}
