use std::{net::SocketAddr, path::PathBuf};

pub struct Config {
    pub listen_addr: SocketAddr,
    pub listen_device: Option<String>,
    pub tls: Vec<TlsConfig>,
}

pub struct TlsConfig {
    pub sni: String,
    pub private_key_file: PathBuf,
    pub certificate_file: PathBuf,
}
