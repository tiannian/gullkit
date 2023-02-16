use std::pin::Pin;

use hyper::{server::conn::Http, service::service_fn};
use openssl::ssl::Ssl;
use tokio::net::TcpStream;
use tokio_openssl::SslStream;

use crate::{handle_http, Result};

pub(crate) async fn accept_https(tcp: TcpStream, ssl: Ssl) -> Result<()> {
    let mut stream = SslStream::new(ssl, tcp)?;

    Pin::new(&mut stream).accept().await?;

    let mut conn = Http::new().serve_connection(stream, service_fn(handle_http));

    Pin::new(&mut conn).graceful_shutdown();

    conn.await?;

    // if let Some(mut parts) = http.try_into_parts() {
    //     parts.io.shutdown().await?;
    // }

    Ok(())
}
