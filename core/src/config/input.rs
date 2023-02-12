use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Input {
    Http,
    Tun,
    Dhcpv4,
    Dhcpv6,
    Dns,
    Socks5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Http {
    domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Tls {
    Acme,
    Local,
}
