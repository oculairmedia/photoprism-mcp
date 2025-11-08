//! PhotoPrism API client implementation
//!
//! This module provides a high-level HTTP client for interacting with the PhotoPrism API.
//! It handles:
//! - Session-based authentication
//! - Token caching and refresh
//! - Generic HTTP methods (GET, POST, PUT, DELETE)
//! - Error handling and retry logic

pub mod auth;

use reqwest::{header, Client, Method, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use crate::config::Config;
use crate::error::{PhotoPrismError, Result};
use auth::{AuthManager, LoginRequest, LoginResponse};

/// PhotoPrism API client with session management
#[derive(Clone)]
pub struct PhotoPrismClient {
    /// HTTP client for making requests
    http_client: Client,

    /// Base URL for the PhotoPrism instance
    base_url: String,

    /// Username for authentication
    username: String,

    /// Password for authentication
    password: String,

    /// Authentication manager for session tokens
    auth_manager: Arc<AuthManager>,
}

impl PhotoPrismClient {
    /// Create a new PhotoPrism API client
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the PhotoPrism instance (e.g., "http://localhost:2342")
    /// * `username` - Username for authentication
    /// * `password` - Password for authentication
    /// * `timeout` - Request timeout duration
    ///
    /// # Returns
    ///
    /// A new PhotoPrismClient instance or an error if client creation fails
    pub fn new(
        base_url: String,
        username: String,
        password: String,
        timeout: Duration,
    ) -> Result<Self> {
        // Build HTTP client with timeout and other settings
        let http_client = Client::builder()
            .timeout(timeout)
            .user_agent("PhotoPrism-MCP/0.1.0")
            .build()
            .map_err(|e| PhotoPrismError::Other(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            http_client,
            base_url: base_url.trim_end_matches('/').to_string(),
            username,
            password,
            auth_manager: Arc::new(AuthManager::new()),
        })
    }

    /// Create a new PhotoPrism API client from configuration
    pub fn from_config(config: &Config) -> Result<Self> {
        let timeout = Duration::from_secs(config.timeout_seconds);
        Self::new(
            config.base_url.clone(),
            config.username.clone(),
            config.password.clone(),
            timeout,
        )
    }

    /// Authenticate with PhotoPrism and obtain a session token
    ///
    /// This method is called automatically by `ensure_authenticated` when needed.
    async fn login(&self) -> Result<String> {
        let login_url = format!("{}/api/v1/session", self.base_url);

        tracing::debug!("Attempting to login to PhotoPrism at {}", login_url);

        let login_request = LoginRequest {
            username: self.username.clone(),
            password: self.password.clone(),
        };

        let response = self
            .http_client
            .post(&login_url)
            .json(&login_request)
            .send()
            .await?;

        // Check response status
        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();

            return Err(PhotoPrismError::AuthenticationFailed(format!(
                "Login failed with status {}: {}",
                status, error_body
            )));
        }

        // Parse login response
        let login_response: LoginResponse = response.json().await.map_err(|e| {
            PhotoPrismError::InvalidResponse(format!("Failed to parse login response: {}", e))
        })?;

        tracing::info!("Successfully authenticated with PhotoPrism");

        Ok(login_response.id)
    }

    /// Ensure we have a valid authentication token
    ///
    /// This checks if we have a cached valid token, and if not, authenticates
    /// to obtain a new one.
    async fn ensure_authenticated(&self) -> Result<String> {
        // Check if we have a valid cached token
        if let Some(token) = self.auth_manager.get_token().await {
            tracing::trace!("Using cached session token");
            return Ok(token);
        }

        // No valid token, need to login
        tracing::debug!("No valid session token, authenticating...");
        let token = self.login().await?;

        // Cache the new token
        self.auth_manager.set_token(token.clone()).await;

        Ok(token)
    }

    /// Make an authenticated HTTP request
    ///
    /// This is a low-level method that handles authentication and token refresh
    async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<Response> {
        // Ensure we have a valid token
        let token = self.ensure_authenticated().await?;

        // Build the full URL
        let url = format!("{}{}", self.base_url, path);

        tracing::trace!("Making {} request to {}", method, url);

        // Build the request
        let mut request = self
            .http_client
            .request(method.clone(), &url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token));

        // Add body if provided
        if let Some(ref body_val) = body {
            request = request.json(body_val);
        }

        // Send the request
        let response = request.send().await?;

        // Handle 401 Unauthorized - token might have expired
        if response.status() == 401 {
            tracing::warn!("Received 401, token may have expired, re-authenticating...");

            // Clear the cached token
            self.auth_manager.clear_token().await;

            // Re-authenticate
            let new_token = self.login().await?;
            self.auth_manager.set_token(new_token.clone()).await;

            // Retry the request with new token
            let mut retry_request = self
                .http_client
                .request(method, &url)
                .header(header::AUTHORIZATION, format!("Bearer {}", new_token));

            if let Some(ref body_val) = body {
                retry_request = retry_request.json(body_val);
            }

            let retry_response = retry_request.send().await?;
            return Ok(retry_response);
        }

        Ok(response)
    }

    /// Make an authenticated GET request
    ///
    /// # Arguments
    ///
    /// * `path` - The API path (e.g., "/api/v1/photos")
    ///
    /// # Returns
    ///
    /// The deserialized response or an error
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self.request(Method::GET, path, None).await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();

            return Err(PhotoPrismError::ApiError(format!(
                "GET {} failed with status {}: {}",
                path, status, error_body
            )));
        }

        response.json().await.map_err(|e| {
            PhotoPrismError::InvalidResponse(format!("Failed to parse response: {}", e))
        })
    }

    /// Make an authenticated POST request
    ///
    /// # Arguments
    ///
    /// * `path` - The API path
    /// * `body` - The request body to serialize as JSON
    ///
    /// # Returns
    ///
    /// The deserialized response or an error
    pub async fn post<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let json_body = serde_json::to_value(body)?;
        let response = self.request(Method::POST, path, Some(json_body)).await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();

            return Err(PhotoPrismError::ApiError(format!(
                "POST {} failed with status {}: {}",
                path, status, error_body
            )));
        }

        response.json().await.map_err(|e| {
            PhotoPrismError::InvalidResponse(format!("Failed to parse response: {}", e))
        })
    }

    /// Make an authenticated PUT request
    ///
    /// # Arguments
    ///
    /// * `path` - The API path
    /// * `body` - The request body to serialize as JSON
    ///
    /// # Returns
    ///
    /// The deserialized response or an error
    pub async fn put<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let json_body = serde_json::to_value(body)?;
        let response = self.request(Method::PUT, path, Some(json_body)).await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();

            return Err(PhotoPrismError::ApiError(format!(
                "PUT {} failed with status {}: {}",
                path, status, error_body
            )));
        }

        response.json().await.map_err(|e| {
            PhotoPrismError::InvalidResponse(format!("Failed to parse response: {}", e))
        })
    }

    /// Make an authenticated DELETE request
    ///
    /// # Arguments
    ///
    /// * `path` - The API path
    ///
    /// # Returns
    ///
    /// Success or an error
    pub async fn delete(&self, path: &str) -> Result<()> {
        let response = self.request(Method::DELETE, path, None).await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();

            return Err(PhotoPrismError::ApiError(format!(
                "DELETE {} failed with status {}: {}",
                path, status, error_body
            )));
        }

        Ok(())
    }

    /// Get the base URL of the PhotoPrism instance
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Check if the client is authenticated (has a valid token)
    pub async fn is_authenticated(&self) -> bool {
        self.auth_manager.has_valid_token().await
    }

    /// Explicitly logout and clear the session token
    pub async fn logout(&self) -> Result<()> {
        if self.is_authenticated().await {
            // Try to call logout endpoint
            let _ = self.delete("/api/v1/session").await;
        }

        // Clear cached token
        self.auth_manager.clear_token().await;

        tracing::info!("Logged out from PhotoPrism");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = PhotoPrismClient::new(
            "http://localhost:2342".to_string(),
            "admin".to_string(),
            "password".to_string(),
            Duration::from_secs(30),
        );

        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.base_url(), "http://localhost:2342");
    }

    #[tokio::test]
    async fn test_client_not_authenticated_initially() {
        let client = PhotoPrismClient::new(
            "http://localhost:2342".to_string(),
            "admin".to_string(),
            "password".to_string(),
            Duration::from_secs(30),
        )
        .unwrap();

        assert!(!client.is_authenticated().await);
    }
}
