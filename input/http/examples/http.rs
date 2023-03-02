use gull_http::{Config, HttpInput};

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config {
        listen_addr: "0.0.0.0:8734".parse().unwrap(),
        listen_device: None,
        http1: true,
        http2: true,
        tls: vec![],
    };

    let http = HttpInput::new((), config).unwrap();
    http.run().await.unwrap();
}
