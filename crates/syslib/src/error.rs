use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyslibError {
    #[error("operation is not supported on this platform")]
    UnsupportedPlatform,

    #[error("invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, SyslibError>;
