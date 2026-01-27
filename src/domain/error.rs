use std::error::Error;
use std::fmt::{self, Display};
use std::io;

/// Library-wide error type capturing domain-neutral and underlying I/O failures.
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    /// Configuration or environment issue that prevents command execution.
    ConfigError(String),
    /// Raised when a requested item cannot be located in storage.
    ItemNotFound(String),
    /// Raised when an item identifier fails validation.
    InvalidItemId(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "{}", err),
            AppError::ConfigError(message) => write!(f, "{message}"),
            AppError::ItemNotFound(id) => write!(f, "Item '{id}' was not found"),
            AppError::InvalidItemId(id) => write!(f, "invalid item identifier: {id}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::ConfigError(_) | AppError::ItemNotFound(_) | AppError::InvalidItemId(_) => {
                None
            }
        }
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError::Io(value)
    }
}

impl AppError {
    pub fn config_error<S: Into<String>>(message: S) -> Self {
        AppError::ConfigError(message.into())
    }

    /// Provide an `io::ErrorKind`-like view for callers expecting legacy behavior.
    pub fn kind(&self) -> io::ErrorKind {
        match self {
            AppError::Io(err) => err.kind(),
            AppError::ConfigError(_) | AppError::InvalidItemId(_) => io::ErrorKind::InvalidInput,
            AppError::ItemNotFound(_) => io::ErrorKind::NotFound,
        }
    }
}
