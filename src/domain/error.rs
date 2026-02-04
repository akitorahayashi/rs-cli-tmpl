use std::io;

/// Library-wide error type capturing domain-neutral and underlying I/O failures.
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Configuration or environment issue that prevents command execution.
    #[error("{0}")]
    ConfigError(String),

    /// Raised when a requested item cannot be located in storage.
    #[error("Item '{0}' was not found")]
    ItemNotFound(String),

    /// Raised when an item identifier fails validation.
    #[error("invalid item identifier: {0}")]
    InvalidItemId(String),
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
