use crate::error::LoggingError;

pub fn verify_root() -> Result<(), LoggingError> {
    if unsafe { libc::geteuid() } != 0 {
        return Err(LoggingError::PermissionError(
            "Root privileges required".to_string(),
        ));
    }
    Ok(())
}