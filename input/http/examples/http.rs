use gull_http::{Config, HttpInput, TlsConfig};

#[tokio::main]
async fn main() {
    env_logger::init();
    openssl::init();

    let config = Config {
        listen_addr: "0.0.0.0:8734".parse().unwrap(),
        listen_device: None,
        tls: vec![],
    };

    let http = HttpInput::new((), config).unwrap();
    http.run().await.unwrap();
}
