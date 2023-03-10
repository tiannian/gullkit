use std::net::SocketAddr;

pub struct Config {
    pub listen_addr: SocketAddr,
    pub listen_device: Option<String>,
    pub tls: Vec<TlsConfig>,
    pub http1: bool,
    pub http2: bool,
}

impl Config {
    pub fn alpn(&self) -> Vec<u8> {
        let mut alpn = Vec::with_capacity(16);

        if self.http2 {
            // alpn.push(2);
            alpn.extend_from_slice(b"h2")
        }

        if self.http1 {
            // alpn.push(8);
            alpn.extend_from_slice(b"http/1.1");
        }

        alpn
    }
}

pub struct TlsConfig {
    pub sni: String,
    pub private_key: String,
    pub certificate: Vec<String>,
}
