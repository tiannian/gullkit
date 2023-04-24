use crate::{
    endpoint_type_len, Endpoint, Error, Protocol, Result, UniversalMessage, UniversalMessageType,
};

pub struct Bytes<T> {
    inner: T,
}

impl<T: AsRef<[u8]>> Bytes<T> {
    fn check(bytes: &[u8]) -> bool {
        false
    }
}

impl<T: AsRef<[u8]>> UniversalMessage for Bytes<T> {
    fn magic(&self) -> Result<[u8; 4]> {
        let r = self.inner.as_ref();

        let a = r.get(0).ok_or(Error::WrongBytesIndex(0))?;
        let b = r.get(1).ok_or(Error::WrongBytesIndex(1))?;
        let c = r.get(2).ok_or(Error::WrongBytesIndex(2))?;
        let d = r.get(3).ok_or(Error::WrongBytesIndex(3))?;

        Ok([*a, *b, *c, *d])
    }

    fn version(&self) -> Result<u8> {
        let r = self.inner.as_ref();

        let a = r.get(4).ok_or(Error::WrongBytesIndex(4))?;

        Ok(*a)
    }

    fn ty(&self) -> Result<UniversalMessageType> {
        let r = self.inner.as_ref();

        let a = r.get(5).ok_or(Error::WrongBytesIndex(5))?;

        Ok(UniversalMessageType::from_u8(*a)?)
    }

    fn connection_id(&self) -> Result<[u8; 32]> {
        let r = self.inner.as_ref();

        build_connection_id(&r[6..])
    }

    fn protocol(&self) -> Result<Protocol> {
        let r = self.inner.as_ref();

        let a = r.get(29).ok_or(Error::WrongBytesIndex(29))?;

        Protocol::from_u8(*a)
    }

    fn code(&self) -> Result<u32> {
        let r = self.inner.as_ref();

        let a = r.get(30).ok_or(Error::WrongBytesIndex(30))?;
        let b = r.get(31).ok_or(Error::WrongBytesIndex(31))?;
        let c = r.get(32).ok_or(Error::WrongBytesIndex(32))?;
        let d = r.get(33).ok_or(Error::WrongBytesIndex(33))?;

        Ok(u32::from_be_bytes([*a, *b, *c, *d]))
    }

    fn from_addr(&self) -> Result<Endpoint> {
        let r = self.inner.as_ref();

        let a = r.get(34).ok_or(Error::WrongBytesIndex(6))?;

        Endpoint::from_u8_and_bytes(*a, &r[1..])
    }

    // fn to_addr(&self) -> Result<Endpoint> {
    //     let r = self.inner.as_ref();
    //
    //     let from_addr_type = *r.get(6).ok_or(Error::WrongBytesIndex(6))?;
    //     let from_addr_len = *r.get(7).ok_or(Error::WrongBytesIndex(7))?;
    //     let from_addr_length = endpoint_type_len(from_addr_type, from_addr_len)?;
    //
    //     let pos = from_addr_length + 6;
    // }
}

fn build_connection_id(data: &[u8]) -> Result<[u8; 32]> {
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

    let a0 = data.get(16).ok_or(Error::WrongBytesIndex(16))?;
    let b0 = data.get(17).ok_or(Error::WrongBytesIndex(17))?;
    let c0 = data.get(18).ok_or(Error::WrongBytesIndex(18))?;
    let d0 = data.get(19).ok_or(Error::WrongBytesIndex(19))?;
    let e0 = data.get(20).ok_or(Error::WrongBytesIndex(20))?;
    let f0 = data.get(21).ok_or(Error::WrongBytesIndex(21))?;
    let g0 = data.get(22).ok_or(Error::WrongBytesIndex(22))?;
    let h0 = data.get(23).ok_or(Error::WrongBytesIndex(23))?;
    let i0 = data.get(24).ok_or(Error::WrongBytesIndex(24))?;
    let j0 = data.get(25).ok_or(Error::WrongBytesIndex(25))?;
    let k0 = data.get(26).ok_or(Error::WrongBytesIndex(26))?;
    let l0 = data.get(27).ok_or(Error::WrongBytesIndex(27))?;
    let m0 = data.get(28).ok_or(Error::WrongBytesIndex(28))?;
    let n0 = data.get(29).ok_or(Error::WrongBytesIndex(29))?;
    let o0 = data.get(30).ok_or(Error::WrongBytesIndex(30))?;
    let p0 = data.get(31).ok_or(Error::WrongBytesIndex(31))?;

    Ok([
        *a, *b, *c, *d, *e, *f, *g, *h, *i, *j, *k, *l, *m, *n, *o, *p, *a0, *b0, *c0, *d0, *e0,
        *f0, *g0, *h0, *i0, *j0, *k0, *l0, *m0, *n0, *o0, *p0,
    ])
}
