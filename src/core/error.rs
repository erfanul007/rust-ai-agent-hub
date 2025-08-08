use anyhow::{Context, Result};

pub trait ErrorContext<T> {
    fn with_context_type(self, context_type: ErrorType, operation: &str) -> Result<T>;
}

impl<T> ErrorContext<T> for Result<T> {
    fn with_context_type(self, context_type: ErrorType, operation: &str) -> Result<T> {
        self.with_context(|| format!("{}: {}", context_type.as_str(), operation))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorType {
    Config,
    Llm,
    Cli,
    Validation,
    Network,
    Io,
}

impl ErrorType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Config => "Configuration error",
            Self::Llm => "LLM operation failed",
            Self::Cli => "CLI operation failed",
            Self::Validation => "Validation failed",
            Self::Network => "Network error",
            Self::Io => "IO error",
        }
    }
}

pub fn app_error(msg: &str) -> anyhow::Error {
    anyhow::anyhow!("Application error: {}", msg)
}

pub fn validation_error(field: &str, reason: &str) -> anyhow::Error {
    anyhow::anyhow!("Validation failed for '{}': {}", field, reason)
}

pub fn context_error<T>(
    result: std::result::Result<T, impl Into<anyhow::Error>>,
    error_type: ErrorType,
    operation: &str,
) -> Result<T> {
    result
        .map_err(Into::into)
        .with_context(|| format!("{}: {}", error_type.as_str(), operation))
}