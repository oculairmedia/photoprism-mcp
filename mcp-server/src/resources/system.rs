use crate::client::PhotoPrismClient;
use std::sync::Arc;
use turbomcp::prelude::*;

/// System resource handlers
pub struct SystemResources {
    pub client: Arc<PhotoPrismClient>,
}

impl SystemResources {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// Get system status
    /// URI: photoprism://system/status
    pub async fn system_status(&self) -> McpResult<String> {
        let status = self
            .client
            .get_status()
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch system status: {}", e)))?;

        Ok(serde_json::to_string_pretty(&status)?)
    }

    /// Get health check
    /// URI: photoprism://system/health
    pub async fn health_check(&self) -> McpResult<String> {
        // Try to get status to verify connectivity
        match self.client.get_status().await {
            Ok(_) => Ok(serde_json::to_string_pretty(&serde_json::json!({
                "status": "healthy",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))?),
            Err(e) => Ok(serde_json::to_string_pretty(&serde_json::json!({
                "status": "unhealthy",
                "error": e.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))?),
        }
    }
}
