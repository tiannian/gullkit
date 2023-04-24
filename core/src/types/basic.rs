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
    CGI,
    DBus,
}

pub enum PayloadRef {
    Bytes(u32),
    Stream(u32),
}
