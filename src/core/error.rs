use config::ConfigError;
use reqwest::Error as ReqwestError;
use std::io::Error as IoError;
use thiserror::Error;

/// Main error type for the application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Configuration(#[from] ConfigurationError),

    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    #[error("Agent error: {0}")]
    Agent(#[from] AgentError),

    #[error("I/O error: {0}")]
    Io(#[from] IoError),
}

/// Configuration-related errors
#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Missing environment variable: {variable}")]
    MissingEnvVar { variable: String },

    #[error("Failed to load configuration file: {0}")]
    FileLoad(#[from] ConfigError),

    #[error("HTTP client configuration failed: {reason}")]
    HttpClient { reason: String },
}

/// Network-related errors
#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] ReqwestError),

    #[error("API request failed with status {status}: {body}")]
    ApiResponse { status: u16, body: String },

    #[error("Failed to parse API response: {reason}")]
    ResponseParsing { reason: String },

    #[error("Request timeout after {seconds} seconds")]
    Timeout { seconds: u64 },
}

/// Agent-related errors
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Agent '{name}' not found. Available agents: {available}")]
    NotFound { name: String, available: String },
}

/// Type alias for Results using our custom error type
pub type Result<T> = std::result::Result<T, AppError>;

/// Helper functions for creating specific errors
impl AppError {
    pub fn missing_env_var(variable: impl Into<String>) -> Self {
        Self::Configuration(ConfigurationError::MissingEnvVar {
            variable: variable.into(),
        })
    }

    pub fn http_client_error(reason: impl Into<String>) -> Self {
        Self::Configuration(ConfigurationError::HttpClient {
            reason: reason.into(),
        })
    }

    pub fn api_response_error(status: u16, body: impl Into<String>) -> Self {
        Self::Network(NetworkError::ApiResponse {
            status,
            body: body.into(),
        })
    }

    pub fn response_parsing_error(reason: impl Into<String>) -> Self {
        Self::Network(NetworkError::ResponseParsing {
            reason: reason.into(),
        })
    }

    pub fn agent_not_found(name: impl Into<String>, available: impl Into<String>) -> Self {
        Self::Agent(AgentError::NotFound {
            name: name.into(),
            available: available.into(),
        })
    }

    pub fn config_load_error(reason: impl Into<String>) -> Self {
        AppError::Configuration(ConfigurationError::FileLoad(
            config::ConfigError::Message(reason.into())
        ))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::Network(NetworkError::Timeout { seconds: 30 })
        } else {
            AppError::Network(NetworkError::Request(err))
        }
    }
}
