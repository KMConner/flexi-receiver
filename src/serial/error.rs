use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("device is not available: {0}")]
    NoDevice(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("malformed packet: {0}")]
    MalformedPacketError(String),

    #[error("unknown error: {0}")]
    UnknownError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
