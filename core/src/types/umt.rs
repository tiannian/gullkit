use crate::{Error, Result};

pub enum UniversalMessageType {
    Packet,
    Request,
    Response,
    SimStream,
    Stream,
}

impl UniversalMessageType {
    pub fn to_u8(self) -> u8 {
        match self {
            UniversalMessageType::Packet => 0x11,
            UniversalMessageType::Request => 0x12,
            UniversalMessageType::Response => 0x13,
            UniversalMessageType::SimStream => 0x21,
            UniversalMessageType::Stream => 0x22,
        }
    }

    pub fn from_u8(v: u8) -> Result<Self> {
        Ok(match v {
            0x11 => Self::Packet,
            0x12 => Self::Request,
            0x13 => Self::Response,
            0x21 => Self::SimStream,
            0x22 => Self::Stream,
            _ => return Err(Error::WrongType("UniversalMessageType", v)),
        })
    }
}
