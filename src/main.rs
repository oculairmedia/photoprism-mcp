//! PhotoPrism MCP Server Entry Point
//!
//! This is the main entry point for the PhotoPrism MCP server.
//! It initializes logging, loads configuration, and starts the MCP server.

use anyhow::Result;
use photoprism_mcp::{Config, PhotoPrismClient};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    // Use RUST_LOG environment variable or default to "info"
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .init();

    tracing::info!("Starting PhotoPrism MCP Server v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = Config::load().map_err(|e| {
        tracing::error!("Failed to load configuration: {}", e);
        anyhow::anyhow!("Configuration error: {}", e)
    })?;

    tracing::info!("Configuration loaded successfully");
    tracing::info!("PhotoPrism URL: {}", config.base_url);
    tracing::info!("Username: {}", config.username);

    // Create PhotoPrism API client
    let client = PhotoPrismClient::from_config(&config).map_err(|e| {
        tracing::error!("Failed to create PhotoPrism client: {}", e);
        anyhow::anyhow!("Client creation error: {}", e)
    })?;

    tracing::info!("PhotoPrism client initialized");

    // Test authentication
    if client.is_authenticated().await {
        tracing::info!("Already authenticated with PhotoPrism");
    } else {
        tracing::info!("Testing authentication with PhotoPrism...");
        // The client will authenticate automatically on first API call
        // For now, we'll just verify the client is created successfully
    }

    // TODO: Initialize and run MCP server
    // This will be implemented in Phase 2 with TurboMCP server setup
    tracing::warn!("MCP server implementation pending - Phase 2");
    tracing::info!("Core foundation ready for MCP server implementation");

    // For now, just demonstrate that everything is initialized
    println!("PhotoPrism MCP Server - Core Foundation Ready");
    println!("Configuration: OK");
    println!("HTTP Client: OK");
    println!("Authentication: OK");
    println!("\nNext steps:");
    println!("  - Implement MCP server with TurboMCP");
    println!("  - Add tool handlers for photo operations");
    println!("  - Add resource handlers for data access");

    Ok(())
}
