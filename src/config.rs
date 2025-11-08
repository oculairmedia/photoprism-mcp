//! Configuration management for PhotoPrism MCP server
//!
//! This module handles loading configuration from multiple sources:
//! 1. Configuration file at ~/.photoprism-mcp/config.yaml
//! 2. Environment variables (as fallback)
//!
//! Priority: Config file > Environment variables > Defaults

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{PhotoPrismError, Result};

/// Configuration for the PhotoPrism MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// PhotoPrism base URL (e.g., http://localhost:2342)
    pub base_url: String,

    /// PhotoPrism username for authentication
    pub username: String,

    /// PhotoPrism password for authentication
    /// Note: This field is not serialized when saving config
    #[serde(skip_serializing)]
    pub password: String,

    /// Optional session token cache (runtime only, not persisted)
    #[serde(skip)]
    pub session_token: Option<String>,

    /// Request timeout in seconds (default: 30)
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,

    /// Enable debug logging
    #[serde(default)]
    pub debug: bool,
}

fn default_timeout() -> u64 {
    30
}

impl Config {
    /// Load configuration from file or environment variables
    ///
    /// Priority:
    /// 1. Config file at ~/.photoprism-mcp/config.yaml (if exists)
    /// 2. Environment variables
    /// 3. Defaults
    pub fn load() -> Result<Self> {
        // Try loading from config file first
        if let Ok(config) = Self::load_from_file() {
            tracing::info!("Loaded configuration from file");
            return Ok(config);
        }

        // Fallback to environment variables
        tracing::info!("Loading configuration from environment variables");
        Self::load_from_env()
    }

    /// Load configuration from the config file
    fn load_from_file() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Err(PhotoPrismError::ConfigError(
                format!("Config file not found at {:?}", config_path)
            ));
        }

        let contents = std::fs::read_to_string(&config_path)
            .map_err(|e| PhotoPrismError::ConfigError(
                format!("Failed to read config file: {}", e)
            ))?;

        let config: Config = serde_yaml::from_str(&contents)?;

        // Validate loaded config
        config.validate()?;

        Ok(config)
    }

    /// Load configuration from environment variables
    fn load_from_env() -> Result<Self> {
        let base_url = std::env::var("PHOTOPRISM_URL")
            .unwrap_or_else(|_| "http://localhost:2342".to_string());

        let username = std::env::var("PHOTOPRISM_USERNAME")
            .unwrap_or_else(|_| "admin".to_string());

        let password = std::env::var("PHOTOPRISM_PASSWORD")
            .map_err(|_| PhotoPrismError::ConfigError(
                "PHOTOPRISM_PASSWORD environment variable not set".to_string()
            ))?;

        let timeout_seconds = std::env::var("PHOTOPRISM_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(default_timeout);

        let debug = std::env::var("PHOTOPRISM_DEBUG")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);

        let config = Config {
            base_url,
            username,
            password,
            session_token: None,
            timeout_seconds,
            debug,
        };

        // Validate config
        config.validate()?;

        Ok(config)
    }

    /// Get the config file path
    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| PhotoPrismError::ConfigError(
                "Cannot determine home directory".to_string()
            ))?
            .join(".photoprism-mcp");

        Ok(config_dir.join("config.yaml"))
    }

    /// Validate configuration values
    fn validate(&self) -> Result<()> {
        // Validate base_url is not empty
        if self.base_url.is_empty() {
            return Err(PhotoPrismError::ConfigError(
                "base_url cannot be empty".to_string()
            ));
        }

        // Validate base_url is a valid URL format
        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err(PhotoPrismError::ConfigError(
                "base_url must start with http:// or https://".to_string()
            ));
        }

        // Validate username is not empty
        if self.username.is_empty() {
            return Err(PhotoPrismError::ConfigError(
                "username cannot be empty".to_string()
            ));
        }

        // Validate password is not empty
        if self.password.is_empty() {
            return Err(PhotoPrismError::ConfigError(
                "password cannot be empty".to_string()
            ));
        }

        // Validate timeout is reasonable (1-300 seconds)
        if self.timeout_seconds < 1 || self.timeout_seconds > 300 {
            return Err(PhotoPrismError::ConfigError(
                "timeout_seconds must be between 1 and 300".to_string()
            ));
        }

        Ok(())
    }

    /// Save configuration to file (excluding password)
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let config_dir = config_path.parent().ok_or_else(|| {
            PhotoPrismError::ConfigError("Invalid config path".to_string())
        })?;

        // Create config directory if it doesn't exist
        std::fs::create_dir_all(config_dir)?;

        // Serialize to YAML
        let yaml = serde_yaml::to_string(self)?;

        // Write to file
        std::fs::write(&config_path, yaml)?;

        tracing::info!("Configuration saved to {:?}", config_path);

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
            timeout_seconds: default_timeout(),
            debug: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        config.password = "test".to_string();

        // Should validate successfully
        assert!(config.validate().is_ok());

        // Test empty base_url
        config.base_url = String::new();
        assert!(config.validate().is_err());

        // Test invalid URL scheme
        config.base_url = "ftp://localhost:2342".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_default_values() {
        let config = Config::default();
        assert_eq!(config.base_url, "http://localhost:2342");
        assert_eq!(config.username, "admin");
        assert_eq!(config.timeout_seconds, 30);
        assert!(!config.debug);
    }
}
