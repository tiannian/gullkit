use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    OpensslErrorStack(#[from] openssl::error::ErrorStack),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
