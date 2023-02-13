use gull_http::{Config, HttpInput, TlsConfig};

#[tokio::main]
async fn main() {
    env_logger::init();
    openssl::init();

    let tls1 = TlsConfig {
        certificate: include_str!("cert.pem.a").to_string(),
        private_key: include_str!("key.pem.a").to_string(),
        sni: String::from("a.gulltoolbox.io"),
    };

    let tls2 = TlsConfig {
        certificate: include_str!("cert.pem.b").to_string(),
        private_key: include_str!("key.pem.b").to_string(),
        sni: String::from("b.gulltoolbox.io"),
    };

    let tls3 = TlsConfig {
        certificate: include_str!("cert.pem.c").to_string(),
        private_key: include_str!("key.pem.c").to_string(),
        sni: String::from("c.gulltoolbox.io"),
    };

    let config = Config {
        listen_addr: "0.0.0.0:8734".parse().unwrap(),
        listen_device: None,
        tls: vec![tls1, tls2, tls3],
    };

    let http = HttpInput::new((), config).unwrap();
    http.run().await.unwrap();
}
