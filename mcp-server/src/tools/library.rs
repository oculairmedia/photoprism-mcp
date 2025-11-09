use crate::client::PhotoPrismClient;
use std::sync::Arc;
use turbomcp::prelude::*;

/// Library management tool handlers
pub struct LibraryTools {
    pub client: Arc<PhotoPrismClient>,
}

impl LibraryTools {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// Get library status
    pub async fn get_status(&self, ctx: Context) -> McpResult<String> {
        ctx.info("Fetching library status").await?;

        let status = self
            .client
            .get_status()
            .await
            .map_err(|e| McpError::internal(format!("Failed to get status: {}", e)))?;

        Ok(serde_json::to_string_pretty(&status)?)
    }
}
