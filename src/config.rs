//! Application configuration for rs-cli-tmpl.

use std::path::PathBuf;

/// Application-wide configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// Base path for storage operations.
    pub storage_path: PathBuf,
}

impl Config {
    /// Create a new configuration with custom storage path.
    pub fn with_path(path: PathBuf) -> Self {
        Self { storage_path: path }
    }
}

impl Default for Config {
    /// Create configuration with default storage path (~/.config/rs-cli-tmpl).
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        Self { storage_path: PathBuf::from(home).join(".config").join("rs-cli-tmpl") }
    }
}
