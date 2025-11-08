//! Authentication and session management for PhotoPrism API
//!
//! This module handles session token management including:
//! - Token caching
//! - Token expiry tracking (24-hour sessions)
//! - Automatic re-authentication

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::error::{PhotoPrismError, Result};

/// Session duration (24 hours as per PhotoPrism API)
const SESSION_DURATION: Duration = Duration::from_secs(24 * 3600);

/// Login request payload
#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: String,
}

/// Login response from PhotoPrism API
#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    /// Session ID/token
    pub id: String,
    /// User information (we only care about the token)
    #[serde(default)]
    pub user: Option<serde_json::Value>,
    /// Session data
    #[serde(default)]
    pub session: Option<serde_json::Value>,
}

/// Session token with expiry tracking
#[derive(Debug, Clone)]
struct SessionToken {
    /// The actual session token
    token: String,
    /// When this token expires
    expires_at: Instant,
}

impl SessionToken {
    /// Create a new session token with current time + 24 hours
    fn new(token: String) -> Self {
        Self {
            token,
            expires_at: Instant::now() + SESSION_DURATION,
        }
    }

    /// Check if the token is still valid
    fn is_valid(&self) -> bool {
        Instant::now() < self.expires_at
    }

    /// Get the token value
    fn token(&self) -> &str {
        &self.token
    }
}

/// Manages authentication state and session tokens
pub struct AuthManager {
    /// Current session token (if any)
    session: Arc<RwLock<Option<SessionToken>>>,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new() -> Self {
        Self {
            session: Arc::new(RwLock::new(None)),
        }
    }

    /// Get the current session token if valid
    ///
    /// Returns None if there's no token or if it has expired
    pub async fn get_token(&self) -> Option<String> {
        let session_guard = self.session.read().await;

        if let Some(session_token) = session_guard.as_ref() {
            if session_token.is_valid() {
                return Some(session_token.token().to_string());
            }
        }

        None
    }

    /// Store a new session token
    ///
    /// This will replace any existing token and set expiry to 24 hours from now
    pub async fn set_token(&self, token: String) {
        let session_token = SessionToken::new(token);
        let mut session_guard = self.session.write().await;
        *session_guard = Some(session_token);

        tracing::debug!("Session token cached, expires in 24 hours");
    }

    /// Clear the current session token
    pub async fn clear_token(&self) {
        let mut session_guard = self.session.write().await;
        *session_guard = None;

        tracing::debug!("Session token cleared");
    }

    /// Check if we have a valid session token
    pub async fn has_valid_token(&self) -> bool {
        let session_guard = self.session.read().await;

        session_guard
            .as_ref()
            .map(|s| s.is_valid())
            .unwrap_or(false)
    }

    /// Get token or return error indicating re-authentication needed
    pub async fn require_token(&self) -> Result<String> {
        self.get_token()
            .await
            .ok_or(PhotoPrismError::TokenExpired)
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auth_manager_new_token() {
        let auth = AuthManager::new();

        // Initially no token
        assert!(auth.get_token().await.is_none());
        assert!(!auth.has_valid_token().await);

        // Set a token
        auth.set_token("test-token-123".to_string()).await;

        // Now we should have a valid token
        assert_eq!(auth.get_token().await, Some("test-token-123".to_string()));
        assert!(auth.has_valid_token().await);
    }

    #[tokio::test]
    async fn test_auth_manager_clear_token() {
        let auth = AuthManager::new();

        // Set and verify token
        auth.set_token("test-token-123".to_string()).await;
        assert!(auth.has_valid_token().await);

        // Clear token
        auth.clear_token().await;

        // Token should be gone
        assert!(auth.get_token().await.is_none());
        assert!(!auth.has_valid_token().await);
    }

    #[tokio::test]
    async fn test_session_token_validity() {
        let token = SessionToken::new("test".to_string());

        // Should be valid immediately
        assert!(token.is_valid());
        assert_eq!(token.token(), "test");
    }

    #[tokio::test]
    async fn test_require_token_when_missing() {
        let auth = AuthManager::new();

        // Should return error when no token
        assert!(matches!(
            auth.require_token().await,
            Err(PhotoPrismError::TokenExpired)
        ));
    }

    #[tokio::test]
    async fn test_require_token_when_present() {
        let auth = AuthManager::new();
        auth.set_token("test-token".to_string()).await;

        // Should return the token
        assert_eq!(auth.require_token().await.unwrap(), "test-token");
    }
}
