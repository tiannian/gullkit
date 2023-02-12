use std::net::SocketAddr;

pub enum Target<'a> {
    SocketAddr(SocketAddr),
    Domain(&'a [u8], u16),
}

pub enum Protocol {
    Http,
    Websocket,
    TCP,
    UDP,
    DNS,
    DHCPv4,
    DHCPv6,
    Socks5,
    Torjan,
    Dynamic,
}

pub enum PayloadRef {
    Bytes(u32),
    Stream(u32),
    File(u32),
}
