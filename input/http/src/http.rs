use std::{net::SocketAddr, result::Result as StdResult, sync::Arc};

use gull_core::Node;
use hyper::server::conn::Http;
use openssl::ssl::{
    NameType, SniError, Ssl, SslAcceptor, SslContext, SslContextBuilder, SslMethod,
};
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

        if config.tls.len() == 0 {
            Ok(Self {
                graph,
                ssl_context: None,
                tcp_listener: listener,
            })
        } else {
            let mut acceptor = SslAcceptor::mozilla_modern(SslMethod::tls())?;

            let configs = Arc::new(utils::build_ssl_context_map(config.tls)?);

            acceptor.set_servername_callback(move |ssl, _| -> StdResult<(), SniError> {
                if let Some(domain) = ssl.servername(NameType::HOST_NAME) {
                    if let Some(ctx) = configs.get(domain) {
                        match ssl.set_ssl_context(ctx) {
                            Ok(()) => Ok(()),
                            Err(e) => {
                                log::error!("{:?}", e);
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
    Graph: Node,
{
    pub async fn run(self) -> Result<()> {
        loop {
            let (conn, addr) = self.tcp_listener.accept().await?;
            log::info!("Receive connection fork {addr}");

            if let Some(ctx) = self.ssl_context.as_ref() {
                let ssl = Ssl::new(ctx)?;
                let stream = SslStream::new(ssl, conn)?;
                incoming(&self.graph, stream).await?;
            } else {
                incoming(&self.graph, conn).await?;
            }
        }
    }
}

pub async fn incoming(
    graph: &impl Node,
    stream: impl AsyncRead + AsyncWrite + Unpin,
) -> Result<()> {
    tokio::spawn(async move {
        if let Err(e) = Http::new().serve_connection(stream, service).await {
            log::error!("{e}");
        }
    });
    Ok(())
}
