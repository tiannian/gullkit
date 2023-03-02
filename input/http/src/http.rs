use std::sync::Arc;

use hyper::{
    server::conn::AddrIncoming,
    service::{make_service_fn, service_fn},
    Server,
};
use tokio::net::TcpListener;

use crate::{handle_http, Error, Result};

pub(crate) async fn run_high_http<G>(tcp: TcpListener, graph: Arc<G>) -> Result<()>
where
    G: Sync + Send + 'static,
{
    let acceptor = AddrIncoming::from_listener(tcp)?;

    let builder = Server::builder(acceptor);

    let mksvc = make_service_fn(move |_| {
        let graph = graph.clone();

        let sf = service_fn(move |req| {
            let graph = graph.clone();

            handle_http(req, graph)
        });

        async move { Ok::<_, Error>(sf) }
    });

    builder.serve(mksvc).await?;
    Ok(())
}

