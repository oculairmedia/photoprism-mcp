//! PhotoPrism MCP Server Library
//!
//! This library provides an MCP (Model Context Protocol) server for PhotoPrism,
//! enabling AI assistants like Claude to interact with PhotoPrism photo libraries.
//!
//! # Features
//!
//! - Session-based authentication with token caching
//! - Type-safe API client for PhotoPrism REST API
//! - Comprehensive error handling
//! - Configuration from files or environment variables
//!
//! # Example
//!
//! ```no_run
//! use photoprism_mcp::{Config, client::PhotoPrismClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load configuration
//!     let config = Config::load()?;
//!
//!     // Create API client
//!     let client = PhotoPrismClient::from_config(&config)?;
//!
//!     // Use the client...
//!     Ok(())
//! }
//! ```

// Public modules
pub mod client;
pub mod config;
pub mod error;

// Re-export commonly used types
pub use config::Config;
pub use client::PhotoPrismClient;
pub use error::{PhotoPrismError, Result};

// Module declarations for future implementation
pub mod server {
    //! MCP server implementation
    //!
    //! This module will contain the TurboMCP server implementation
}

pub mod tools {
    //! Tool implementations for MCP
    //!
    //! This module will contain tool handlers for photo/album operations
}

pub mod resources {
    //! Resource handlers for MCP
    //!
    //! This module will contain resource handlers for read-only data access
}

pub mod types {
    //! Common types and models
    //!
    //! This module will contain PhotoPrism API types and shared models
}
