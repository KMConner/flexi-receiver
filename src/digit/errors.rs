use thiserror::Error as ThisError;
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse packet: {0}")]
    DigitParseError(String)
}

pub type Result<T> = std::result::Result<T, Error>;
