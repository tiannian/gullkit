use crate::{Error, Result};

pub enum Endpoint<'a> {
    Ipv4Addr([u8; 4]),
    Ipv6Addr([u8; 16]),
    SocketAddrV4([u8; 4], u16),
    SocketAddrV6([u8; 16], u16),
    DomainNameWithPort(&'a [u8], u16),
    Ethernet([u8; 6]),
}

pub(crate) fn endpoint_type_len(v: u8, len: u8) -> Result<usize> {
    Ok(match v {
        0x10 => 4,
        0x11 => 16,
        0x20 => 6,
        0x21 => 18,
        0x30 => 6,
        0x40 => len as usize,
        _ => return Err(Error::WrongType("Endpoint", v)),
    })
}

impl<'a> Endpoint<'a> {
    pub fn from_u8_and_bytes(ty: u8, data: &'a [u8]) -> Result<Endpoint<'a>> {
        match ty {
            0x10 => Ok(Endpoint::Ipv4Addr(build_ipv4(data)?)),
            0x11 => Ok(Endpoint::Ipv6Addr(build_ipv6(data)?)),
            0x20 => {
                let a = data.get(16).ok_or(Error::WrongBytesIndex(16))?;
                let b = data.get(17).ok_or(Error::WrongBytesIndex(17))?;

                let port = u16::from_be_bytes([*a, *b]);

                Ok(Endpoint::SocketAddrV4(build_ipv4(data)?, port))
            }
            0x21 => {
                let a = data.get(16).ok_or(Error::WrongBytesIndex(16))?;
                let b = data.get(17).ok_or(Error::WrongBytesIndex(17))?;

                let port = u16::from_be_bytes([*a, *b]);

                Ok(Endpoint::SocketAddrV6(build_ipv6(data)?, port))
            }
            0x30 => Ok(Endpoint::Ethernet(build_ether(data)?)),
            0x40 => {
                let length = *data.get(0).ok_or(Error::WrongBytesIndex(0))? as usize;

                let a = data.get(length).ok_or(Error::WrongBytesIndex(length))?;
                let lengthoff = length + 1;
                let b = data
                    .get(lengthoff)
                    .ok_or(Error::WrongBytesIndex(lengthoff))?;

                let port = u16::from_be_bytes([*a, *b]);

                Ok(Endpoint::DomainNameWithPort(&data[1..lengthoff], port))
            }
            _ => Err(Error::WrongType("EndpointType", ty)),
        }
    }
}

fn build_ipv4(data: &[u8]) -> Result<[u8; 4]> {
    let a = data.get(0).ok_or(Error::WrongBytesIndex(0))?;
    let b = data.get(1).ok_or(Error::WrongBytesIndex(1))?;
    let c = data.get(2).ok_or(Error::WrongBytesIndex(2))?;
    let d = data.get(3).ok_or(Error::WrongBytesIndex(3))?;

    Ok([*a, *b, *c, *d])
}

fn build_ipv6(data: &[u8]) -> Result<[u8; 16]> {
    let a = data.get(0).ok_or(Error::WrongBytesIndex(0))?;
    let b = data.get(1).ok_or(Error::WrongBytesIndex(1))?;
    let c = data.get(2).ok_or(Error::WrongBytesIndex(2))?;
    let d = data.get(3).ok_or(Error::WrongBytesIndex(3))?;
    let e = data.get(4).ok_or(Error::WrongBytesIndex(4))?;
    let f = data.get(5).ok_or(Error::WrongBytesIndex(5))?;
    let g = data.get(6).ok_or(Error::WrongBytesIndex(6))?;
    let h = data.get(7).ok_or(Error::WrongBytesIndex(7))?;
    let i = data.get(8).ok_or(Error::WrongBytesIndex(8))?;
    let j = data.get(9).ok_or(Error::WrongBytesIndex(9))?;
    let k = data.get(10).ok_or(Error::WrongBytesIndex(10))?;
    let l = data.get(11).ok_or(Error::WrongBytesIndex(11))?;
    let m = data.get(12).ok_or(Error::WrongBytesIndex(12))?;
    let n = data.get(13).ok_or(Error::WrongBytesIndex(13))?;
    let o = data.get(14).ok_or(Error::WrongBytesIndex(14))?;
    let p = data.get(15).ok_or(Error::WrongBytesIndex(15))?;

    Ok([
        *a, *b, *c, *d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p,
    ])
}

fn build_ether(data: &[u8]) -> Result<[u8; 6]> {
    let a = data.get(0).ok_or(Error::WrongBytesIndex(0))?;
    let b = data.get(1).ok_or(Error::WrongBytesIndex(1))?;
    let c = data.get(2).ok_or(Error::WrongBytesIndex(2))?;
    let d = data.get(3).ok_or(Error::WrongBytesIndex(3))?;
    let e = data.get(4).ok_or(Error::WrongBytesIndex(4))?;
    let f = data.get(5).ok_or(Error::WrongBytesIndex(5))?;

    Ok([*a, *b, *c, *d, *e, *f])
}
