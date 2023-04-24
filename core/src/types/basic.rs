use crate::Result;

pub enum Protocol {
    Http,
    Websocket,
    TCP,
    UDP,
    DNS,
    DHCPv4,
    DHCPv6,
    Socks5,
    Trojan,
    IP,
    Ethernet,
    CGI,
    DBus,
}

impl Protocol {
    pub fn from_u8(v: u8) -> Result<Protocol> {
        Ok(match v {
            1 => Self::Http,
            2 => Self::Websocket,
            3 => Self::TCP,
            4 => Self::UDP,
            5 => Self::DNS,
            6 => Self::DHCPv4,
            7 => Self::DHCPv6,
            8 => Self::Socks5,
            9 => Self::Trojan,
            10 => Self::IP,
            11 => Self::Ethernet,
            12 => Self::CGI,
            13 => Self::DBus,
        })
    }
}

pub enum PayloadRef {
    Bytes(u32),
    Stream(u32),
}
