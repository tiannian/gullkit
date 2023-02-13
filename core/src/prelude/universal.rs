use crate::{Endpoint, PayloadRef, Protocol};

pub trait UniversalMessage {
    type OptionIter<'a>: Iterator<Item = (&'a [u8], &'a [u8])>
    where
        Self: 'a;

    fn magic(&self) -> &[u8; 4];

    fn version(&self) -> u8;

    fn from(&self) -> Endpoint;

    fn to(&self) -> Endpoint;

    fn protocol(&self) -> Protocol;

    fn code(&self) -> u8;

    fn path(&self) -> &[u8];

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
