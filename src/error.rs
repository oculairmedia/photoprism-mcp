//! Error types for PhotoPrism MCP server
//!
//! This module defines custom error types for the PhotoPrism MCP server
//! and provides conversions to MCP protocol errors.

use thiserror::Error;

/// Custom error type for PhotoPrism operations
#[derive(Error, Debug)]
pub enum PhotoPrismError {
    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// API request failed
    #[error("API request failed: {0}")]
    ApiError(String),

    /// Resource not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Invalid parameter provided
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Network error occurred
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// YAML serialization/deserialization error
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Session token expired
    #[error("Session token expired")]
    TokenExpired,

    /// Invalid response from PhotoPrism API
    #[error("Invalid API response: {0}")]
    InvalidResponse(String),

    /// Generic error for wrapping other errors
    #[error("Error: {0}")]
    Other(String),
}

/// Convert PhotoPrismError to turbomcp_protocol::Error
impl From<PhotoPrismError> for Box<turbomcp_protocol::Error> {
    fn from(err: PhotoPrismError) -> Self {
        match err {
            PhotoPrismError::AuthenticationFailed(msg) => {
                turbomcp_protocol::Error::authentication(msg)
            }
            PhotoPrismError::NotFound(msg) => {
                turbomcp_protocol::Error::resource_not_found(msg)
            }
            PhotoPrismError::InvalidParameter(msg) => {
                turbomcp_protocol::Error::invalid_params(msg)
            }
            PhotoPrismError::TokenExpired => {
                turbomcp_protocol::Error::authentication("Session token expired")
            }
            PhotoPrismError::ApiError(msg) => {
                turbomcp_protocol::Error::external_service(msg)
            }
            PhotoPrismError::NetworkError(e) => {
                turbomcp_protocol::Error::transport(format!("Network error: {}", e))
            }
            PhotoPrismError::ConfigError(msg) => {
                turbomcp_protocol::Error::configuration(msg)
            }
            _ => turbomcp_protocol::Error::internal(err.to_string()),
        }
    }
}

/// Result type alias for PhotoPrism operations
pub type Result<T> = std::result::Result<T, PhotoPrismError>;
