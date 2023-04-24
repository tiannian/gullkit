use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("wrong bytes index {0}")]
    WrongBytesIndex(u8),

    #[error("wrong type {0} {1}")]
    WrongType(&'static str, u8),
}

pub type Result<T> = core::result::Result<T, Error>;
