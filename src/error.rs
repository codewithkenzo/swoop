/*!
 * Error handling module for Crawl4AI
 * 
 * This module defines the error types and result type used throughout the Crawl4AI system.
 */

use std::fmt;
use thiserror::Error;

/// Result type for Crawl4AI operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for Crawl4AI
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    /// URL parsing error
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
    
    /// I/O error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON error
    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    
    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    /// Parser error  
    #[error("Parsing error: {0}")]
    Parsing(String),
    
    /// Parser error (alternative name)
    #[error("Parser error: {0}")]
    Parser(String),
    
    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),
    
    /// Vector storage error
    #[error("Vector storage error: {0}")]
    VectorStorage(String),
    
    /// Robots.txt error
    #[error("Robots.txt error: {0}")]
    Robots(String),
    
    /// Headless browser error
    #[error("Headless browser error: {0}")]
    HeadlessBrowser(String),
    
    /// LLM API error
    #[error("LLM API error: {0}")]
    LlmApi(String),
    
    /// GPU error
    #[error("GPU error: {0}")]
    Gpu(String),
    
    /// Timeout error
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    /// Concurrency error
    #[error("Concurrency error: {0}")]
    Concurrency(String),
    
    /// Rate limit error
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    /// Authorization error
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    /// Not found error
    #[error("Not found error: {0}")]
    NotFound(String),
    
    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),
    
    /// Conversion error
    #[error("Conversion error: {0}")]
    Conversion(String),
    
    /// External service error
    #[error("External service error: {0}")]
    ExternalService(String),
    
    /// MCP error
    #[error("MCP error: {0}")]
    Mcp(String),
    
    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

impl Error {
    /// Returns true if the error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Http(e) => {
                // Retry on server errors (5xx) and some specific client errors
                if let Some(status) = e.status() {
                    status.is_server_error() || status.as_u16() == 429
                } else {
                    // Retry on network errors
                    e.is_timeout() || e.is_connect()
                }
            }
            Error::Timeout(_) => true,
            Error::RateLimit(_) => true,
            Error::ExternalService(_) => true,
            Error::Concurrency(_) => true,
            _ => false,
        }
    }
    
    /// Returns the error code
    pub fn code(&self) -> String {
        match self {
            Error::Configuration(_) => "CONFIGURATION_ERROR".to_string(),
            Error::Http(_) => "HTTP_ERROR".to_string(),
            Error::UrlParse(_) => "URL_PARSE_ERROR".to_string(),
            Error::Io(_) => "IO_ERROR".to_string(),
            Error::SerdeJson(_) => "JSON_ERROR".to_string(),
            Error::Database(_) => "DATABASE_ERROR".to_string(),
            Error::Parsing(_) => "PARSER_ERROR".to_string(),
            Error::Parser(_) => "PARSER_ERROR".to_string(),
            Error::Storage(_) => "STORAGE_ERROR".to_string(),
            Error::VectorStorage(_) => "VECTOR_STORAGE_ERROR".to_string(),
            Error::Robots(_) => "ROBOTS_ERROR".to_string(),
            Error::HeadlessBrowser(_) => "HEADLESS_BROWSER_ERROR".to_string(),
            Error::LlmApi(_) => "LLM_API_ERROR".to_string(),
            Error::Gpu(_) => "GPU_ERROR".to_string(),
            Error::Timeout(_) => "TIMEOUT_ERROR".to_string(),
            Error::Concurrency(_) => "CONCURRENCY_ERROR".to_string(),
            Error::RateLimit(_) => "RATE_LIMIT_ERROR".to_string(),
            Error::Authentication(_) => "AUTHENTICATION_ERROR".to_string(),
            Error::Authorization(_) => "AUTHORIZATION_ERROR".to_string(),
            Error::NotFound(_) => "NOT_FOUND_ERROR".to_string(),
            Error::Validation(_) => "VALIDATION_ERROR".to_string(),
            Error::Conversion(_) => "CONVERSION_ERROR".to_string(),
            Error::ExternalService(_) => "EXTERNAL_SERVICE_ERROR".to_string(),
            Error::Mcp(_) => "MCP_ERROR".to_string(),
            Error::Other(_) => "OTHER_ERROR".to_string(),
        }
    }
    
    /// Returns a structured error response
    pub fn to_response(&self) -> ErrorResponse {
        ErrorResponse {
            code: self.code(),
            message: self.to_string(),
            details: None,
        }
    }
}

/// Structured error response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Additional details
    pub details: Option<serde_json::Value>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

/// Convert String to Error
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

/// Convert &str to Error
impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}

/// Convert regex::Error to Error
impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Other(format!("Regex error: {err}"))
    }
}

/// Convert tokio::sync::AcquireError to Error
impl From<tokio::sync::AcquireError> for Error {
    fn from(err: tokio::sync::AcquireError) -> Self {
        Error::Concurrency(err.to_string())
    }
}

/// Convert redis::RedisError to Error
impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::ExternalService(format!("Redis error: {err}"))
    }
}

/// Convert sqlx::migrate::MigrateError to Error
impl From<sqlx::migrate::MigrateError> for Error {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        Error::Storage(format!("Migration error: {e}"))
    }
}

// Pool timeout and closed errors are handled through the main sqlx::Error enum

// These From implementations are handled by the #[from] derive macro above

// Legacy alias for backward compatibility
pub type SwoopError = Error;
