use std::net::SocketAddr;

use crate::{PayloadRef, Protocol, Target};

pub trait UniversalMessage {
    type OptionIter<'a>: Iterator<Item = (&'a [u8], &'a [u8])>
    where
        Self: 'a;

    fn magic(&self) -> &[u8; 4];

    fn version(&self) -> u8;

    fn from(&self) -> SocketAddr;

    fn to(&self) -> Target;

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

    fn set_magic(&self, magic: &[u8; 32]);

    fn set_version(&self, version: u8);

    fn set_from(&self, from: SocketAddr);

    fn set_to(&self, to: Target);

    fn set_code(&self, code: u8);

    fn set_protocol(&self, protocol: Protocol);

    fn path_mut(&self) -> &mut [u8];

    fn options_mut(&self) -> Self::OptionIterMut<'_>;

    fn payload_mut(&self, payload: PayloadRef);
}
