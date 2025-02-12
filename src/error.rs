use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggingError {
    #[error("Profile operation failed: {0}")]
    ProfileError(String),
    #[error("System command failed: {0}")]
    CommandError(String),
    #[error("IO operation failed: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Permission denied: {0}")]
    PermissionError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}