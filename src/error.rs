use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid operation. ")]
    ParsingError,
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::InvalidOperation(s)
    }
}
