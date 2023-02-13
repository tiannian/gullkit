use std::net::SocketAddr;

pub struct Config {
    pub listen_addr: SocketAddr,
    pub listen_device: Option<String>,
    pub tls: Vec<TlsConfig>,
}

pub struct TlsConfig {
    pub sni: String,
    pub private_key: String,
    pub certificate: String,
}
