use thiserror::Error;

/// PhotoPrism MCP Server errors
#[derive(Error, Debug)]
pub enum PhotoPrismError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

/// Convert PhotoPrismError to turbomcp McpError
impl From<PhotoPrismError> for turbomcp::McpError {
    fn from(err: PhotoPrismError) -> Self {
        // Use McpError::internal for all error types
        // TurboMCP will handle the error message appropriately
        turbomcp::McpError::internal(err.to_string())
    }
}

/// Result type for PhotoPrism operations
pub type Result<T> = std::result::Result<T, PhotoPrismError>;
