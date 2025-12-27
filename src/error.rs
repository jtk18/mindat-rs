//! Error types for the Mindat API client.

use thiserror::Error;

/// Errors that can occur when using the Mindat API client.
#[derive(Error, Debug)]
pub enum MindatError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// Failed to parse URL
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),

    /// API returned an error response
    #[error("API error (status {status}): {message}")]
    Api { status: u16, message: String },

    /// Failed to deserialize response
    #[error("Failed to parse response: {0}")]
    Deserialization(#[from] serde_json::Error),

    /// Authentication error - missing or invalid token
    #[error("Authentication required: please provide a valid API token")]
    AuthenticationRequired,

    /// Rate limit exceeded
    #[error("Rate limit exceeded, please wait before making more requests")]
    RateLimited,

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Invalid parameter value
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

/// Result type alias for Mindat operations.
pub type Result<T> = std::result::Result<T, MindatError>;
