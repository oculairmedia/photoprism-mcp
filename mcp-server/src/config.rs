use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{PhotoPrismError, Result};

/// Configuration for PhotoPrism MCP Server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// PhotoPrism base URL (e.g., http://localhost:2342)
    pub base_url: String,

    /// PhotoPrism username
    pub username: String,

    /// PhotoPrism password
    #[serde(skip_serializing)]
    pub password: String,

    /// Session token (cached, not persisted)
    #[serde(skip)]
    pub session_token: Option<String>,
}

impl Config {
    /// Load configuration from file or environment variables
    pub fn load() -> Result<Self> {
        // Try loading from ~/.config/photoprism-mcp/config.yaml
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir
                .join("photoprism-mcp")
                .join("config.yaml");

            if config_path.exists() {
                let contents = std::fs::read_to_string(&config_path)?;
                return Ok(serde_yaml::from_str(&contents)?);
            }
        }

        // Fallback to environment variables
        let base_url = std::env::var("PHOTOPRISM_URL")
            .unwrap_or_else(|_| "http://localhost:2342".to_string());

        let username = std::env::var("PHOTOPRISM_USERNAME")
            .unwrap_or_else(|_| "admin".to_string());

        let password = std::env::var("PHOTOPRISM_PASSWORD")
            .map_err(|_| PhotoPrismError::ConfigError(
                "PHOTOPRISM_PASSWORD environment variable not set".to_string()
            ))?;

        Ok(Self {
            base_url,
            username,
            password,
            session_token: None,
        })
    }

    /// Create a new config from explicit values
    pub fn new(base_url: String, username: String, password: String) -> Self {
        Self {
            base_url,
            username,
            password,
            session_token: None,
        }
    }

    /// Get config directory path
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("photoprism-mcp"))
    }

    /// Save configuration to file (without password)
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()
            .ok_or_else(|| PhotoPrismError::ConfigError(
                "Cannot find config directory".to_string()
            ))?;

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.yaml");
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(&config_path, yaml)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:2342".to_string(),
            username: "admin".to_string(),
            password: String::new(),
            session_token: None,
        }
    }
}
