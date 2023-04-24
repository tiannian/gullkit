use crate::{endpoint_type_len, Endpoint, Error, Result, UniversalMessage, UniversalMessageType};

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

    fn from_addr(&self) -> Result<Endpoint> {
        let r = self.inner.as_ref();

        let a = r.get(6).ok_or(Error::WrongBytesIndex(6))?;

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
