use anyhow::Result;
use photoprism_mcp::{Config, PhotoPrismServer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    tracing::info!("Starting PhotoPrism MCP Server");

    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            eprintln!("\nPlease set the following environment variables:");
            eprintln!("  PHOTOPRISM_URL      - PhotoPrism server URL (default: http://localhost:2342)");
            eprintln!("  PHOTOPRISM_USERNAME - PhotoPrism username (default: admin)");
            eprintln!("  PHOTOPRISM_PASSWORD - PhotoPrism password (required)");
            eprintln!("\nOr create a config file at: ~/.config/photoprism-mcp/config.yaml");
            eprintln!("\nExample config.yaml:");
            eprintln!("  base_url: http://localhost:2342");
            eprintln!("  username: admin");
            eprintln!("  # password is loaded from PHOTOPRISM_PASSWORD env var");
            std::process::exit(1);
        }
    };

    tracing::info!("Connecting to PhotoPrism at: {}", config.base_url);

    // Create and run server
    let server = PhotoPrismServer::new(config)?;

    tracing::info!("Starting PhotoPrism MCP server with STDIO transport");

    // Run server with STDIO transport
    server.run_stdio().await
        .map_err(|e| anyhow::anyhow!("Server failed: {}", e))?;

    Ok(())
}
