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
    LanguageModel,
    CommandLineInterface,
    InputValidation,
    NetworkOperation,
    FileSystem,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Configuration => "Configuration error",
            Self::LanguageModel => "Language model operation failed",
            Self::CommandLineInterface => "CLI operation failed",
            Self::InputValidation => "Input validation failed",
            Self::NetworkOperation => "Network operation failed",
            Self::FileSystem => "File system operation failed",
        };
        write!(f, "{}", message)
    }
}

pub fn create_application_error(message: &str) -> anyhow::Error {
    anyhow::anyhow!("Application error: {}", message)
}

pub fn create_validation_error(field: &str, reason: &str) -> anyhow::Error {
    anyhow::anyhow!("Validation failed for '{}': {}", field, reason)
}

pub fn wrap_with_context<T>(
    result: std::result::Result<T, impl Into<anyhow::Error>>,
    error_category: ErrorCategory,
    operation: &str,
) -> Result<T> {
    result
        .map_err(Into::into)
        .with_context(|| format!("{}: {}", error_category, operation))
}