use thiserror::{Error as ThisError, Error};

#[derive(ThisError, Debug, PartialEq)]
pub enum Error {
    #[error("failed to parse packet: {0}")]
    DigitParseError(String),

    #[error("Decimal point can exist only in the second digit")]
    InvalidDecimalPointError,
}

pub type Result<T> = std::result::Result<T, Error>;
