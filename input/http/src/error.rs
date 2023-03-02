use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "openssl-tls")]
    #[error(transparent)]
    OpensslErrorStack(#[from] openssl::error::ErrorStack),

    #[cfg(feature = "openssl-tls")]
    #[error(transparent)]
    OpensslError(#[from] openssl::ssl::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error("Failed to parse PEM file")]
    ParsePEMFaile,

    #[cfg(feature = "rustls-tls")]
    #[error(transparent)]
    RustlsSignError(#[from] rustls::sign::SignError),

    #[cfg(feature = "rustls-tls")]
    #[error(transparent)]
    RustlsError(#[from] rustls::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
