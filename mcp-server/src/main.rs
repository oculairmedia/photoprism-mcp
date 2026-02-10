use anyhow::Result;
use photoprism_mcp::{Config, PhotoPrismServer};
use tracing_subscriber::EnvFilter;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging with explicit stderr and try_init to prevent Docker panics
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .try_init();

    eprintln!("Starting PhotoPrism MCP Server");

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

    eprintln!("Connecting to PhotoPrism at: {}", config.base_url);

    // Create and run server
    let server = PhotoPrismServer::new(config)?;

    // Determine transport mode from environment variable
    let transport = env::var("TRANSPORT").unwrap_or_else(|_| "stdio".to_string());
    
    match transport.to_lowercase().as_str() {
        "http" => {
            let port = env::var("HTTP_PORT").unwrap_or_else(|_| "3000".to_string());
            let addr = format!("0.0.0.0:{}", port);
            eprintln!("🚀 Starting HTTP transport");
            eprintln!("📡 Listening on: http://{}", addr);
            eprintln!("🔗 Endpoint: http://{}/mcp", addr);
            eprintln!("Ready for MCP client connections");
            
            // Use turbomcp_transport streamable HTTP with permissive security for development
            use turbomcp_transport::streamable_http_v2::{StreamableHttpConfigBuilder, run_server};
            use std::sync::Arc;
            use std::time::Duration;
            
            let config = StreamableHttpConfigBuilder::new()
                .with_bind_address(&addr)
                .allow_any_origin(true)  // Allow any origin in development mode
                .allow_localhost(true)
                .with_rate_limit(1_000_000, Duration::from_secs(60))  // Very high limit for development
                .build();
            
            run_server(config, Arc::new(server))
                .await
                .map_err(|e| anyhow::anyhow!("HTTP server failed: {}", e))?;
        }
        _ => {
            eprintln!("🚀 Starting STDIO transport");
            eprintln!("Ready for MCP client connections");
            
            server
                .run_stdio()
                .await
                .map_err(|e| anyhow::anyhow!("STDIO server failed: {}", e))?;
        }
    }

    Ok(())
}
