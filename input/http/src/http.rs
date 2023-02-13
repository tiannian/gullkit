use std::{net::SocketAddr, pin::Pin, result::Result as StdResult, sync::Arc};

use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use openssl::ssl::{NameType, SniError, Ssl, SslAcceptor, SslContext, SslMethod};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpSocket},
};
use tokio_openssl::SslStream;

use crate::{utils, Config, Result};

pub struct HttpInput<Graph> {
    graph: Graph,
    ssl_context: Option<SslContext>,
    tcp_listener: TcpListener,
}

impl<Graph> HttpInput<Graph> {
    pub fn new(graph: Graph, config: Config) -> Result<Self> {
        let listener = match &config.listen_addr {
            SocketAddr::V4(_) => {
                let socket = TcpSocket::new_v4()?;
                socket.bind(config.listen_addr)?;
                socket
            }
            SocketAddr::V6(_) => {
                let socket = TcpSocket::new_v6()?;
                socket.bind(config.listen_addr)?;
                socket
            }
        };

        if let Some(device) = &config.listen_device {
            listener.bind_device(Some(device.as_bytes()))?;
        }

        listener.set_reuseport(true)?;
        listener.set_reuseaddr(true)?;

        let listener = listener.listen(1024)?;

        if config.tls.is_empty() {
            Ok(Self {
                graph,
                ssl_context: None,
                tcp_listener: listener,
            })
        } else {
            let mut acceptor = SslAcceptor::mozilla_modern(SslMethod::tls())?;

            // utils::set_context_builder(
            //     &mut acceptor,
            //     &config.tls[0].private_key,
            //     &config.tls[0].certificate,
            // )?;

            let configs = Arc::new(utils::build_ssl_context_map(config.tls)?);

            acceptor.set_servername_callback(move |ssl, _| -> StdResult<(), SniError> {
                if let Some(domain) = ssl.servername(NameType::HOST_NAME) {
                    if let Some(ctx) = configs.get(domain) {
                        log::info!("Receive sni: {domain}");

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

            let ssl_context = acceptor.build().into_context();

            Ok(Self {
                graph,
                ssl_context: Some(ssl_context),
                tcp_listener: listener,
            })
        }
    }
}

impl<Graph> HttpInput<Graph>
where
    Graph: Sync + Send + 'static,
{
    pub async fn run(self) -> Result<()> {
        let graph = Arc::new(self.graph);

        loop {
            let (conn, addr) = self.tcp_listener.accept().await?;
            log::info!("Receive connection from: {addr}");

            if let Some(ctx) = &self.ssl_context {
                // TODO: ignore error.
                let ssl = Ssl::new(ctx)?;
                let mut stream = SslStream::new(ssl, conn)?;

                Pin::new(&mut stream).accept().await?;

                incoming(graph.clone(), stream).await?;
            } else {
                incoming(graph.clone(), conn).await?;
            }
        }
    }
}

pub async fn incoming<G, S>(graph: Arc<G>, stream: S) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    G: Sync + Send + 'static,
{
    tokio::spawn(async move {
        process_stream(graph.as_ref(), stream).await;
    });

    Ok(())
}

pub async fn process_stream<N, S>(graph: &N, stream: S)
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let service = service_fn(|req: Request<Body>| async move { handle_requets(req).await });

    if let Err(e) = Http::new().serve_connection(stream, service).await {
        log::error!("handle http connection failed: {e}")
    }
}

pub async fn handle_requets(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello !")))
}
