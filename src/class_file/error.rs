use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("Invalid class file. {message:}")]
pub struct Error {
    pub message: String,
}

// utils
pub fn error<T>(message: String) -> Result<T> {
    Err(Error { message })
}