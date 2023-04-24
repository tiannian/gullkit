use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("wrong bytes index {0}")]
    WrongBytesIndex(usize),

    #[error("wrong type {0} {1}")]
    WrongType(&'static str, u8),
}

pub type Result<T> = core::result::Result<T, Error>;
