use anyhow::{Context, Result};
use std::fmt;

pub trait ErrorContextExt<T> {
    fn with_operation_context(self, error_type: ErrorCategory, operation: &str) -> Result<T>;
}

impl<T> ErrorContextExt<T> for Result<T> {
    fn with_operation_context(self, error_type: ErrorCategory, operation: &str) -> Result<T> {
        self.with_context(|| format!("{}: {}", error_type, operation))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorCategory {
    Configuration,
    NetworkOperation,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Configuration => "Configuration error",
            Self::NetworkOperation => "Network operation failed",
        };
        write!(f, "{}", message)
    }
}

pub fn create_application_error(message: &str) -> anyhow::Error {
    anyhow::anyhow!("Application error: {}", message)
}