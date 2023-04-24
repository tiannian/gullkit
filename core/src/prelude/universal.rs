use crate::{Endpoint, PayloadRef, Protocol, Result, UniversalMessageType};

pub trait UniversalMessage {
    type OptionIter<'a>: Iterator<Item = (&'a [u8], &'a [u8])>
    where
        Self: 'a;

    fn magic(&self) -> Result<[u8; 4]>;

    fn version(&self) -> Result<u8>;

    fn ty(&self) -> Result<UniversalMessageType>;

    fn from_addr(&self) -> Result<Endpoint>;

    fn to_addr(&self) -> Result<Endpoint>;

    fn connection_id(&self) -> Result<[u8; 32]>;

    fn protocol(&self) -> Result<Protocol>;

    fn code(&self) -> Result<u8>;

    fn path(&self) -> Result<&[u8]>;

    fn options(&self) -> Self::OptionIter<'_>;

    fn payload_ref(&self) -> PayloadRef;
}

pub trait UniversalMessageMut: UniversalMessage {
    type OptionIterMut<'a>: Iterator<Item = (&'a mut [u8], &'a mut [u8])>
    where
        Self: 'a;

    fn set_magic(&mut self, magic: &[u8; 32]);

    fn set_version(&mut self, version: u8);

    fn set_from(&mut self, from: Endpoint);

    fn set_to(&mut self, to: Endpoint);

    fn set_code(&mut self, code: u8);

    fn set_protocol(&mut self, protocol: Protocol);

    fn path_mut(&mut self) -> &mut [u8];

    fn options_mut(&mut self) -> Self::OptionIterMut<'_>;

    fn payload_mut(&mut self, payload: PayloadRef);
}
