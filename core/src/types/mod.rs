pub enum Endpoint<'a> {
    SocketAddrV4(&'a [u8; 4], u16),
    SocketAddrV6(&'a [u8; 16], u16),
    DomainName(&'a [u8], u16),
    Ethernet(&'a [u8; 6]),
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
    Ip,
    Ethernet,
}

pub enum PayloadRef {
    Bytes(u32),
    Stream(u32),
}
