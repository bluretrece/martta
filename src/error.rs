use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid operation. ")]
    ParsingError,
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Types mismatch: {0}")]
    TypeError(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::InvalidOperation(s)
    }
}
