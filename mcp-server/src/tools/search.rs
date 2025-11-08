use turbomcp::prelude::*;
use crate::client::PhotoPrismClient;
use std::sync::Arc;

/// Search tool handlers
pub struct SearchTools {
    pub client: Arc<PhotoPrismClient>,
}

impl SearchTools {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// Search photos with text query
    pub async fn search_photos(
        &self,
        ctx: Context,
        query: String,
        count: Option<u32>,
    ) -> McpResult<String> {
        let count = count.unwrap_or(100).min(1000);

        ctx.info(&format!("Searching photos with query: '{}'", query)).await?;

        let photos = self.client
            .search_photos(&query, count)
            .await
            .map_err(|e| McpError::internal(format!("Search failed: {}", e)))?;

        ctx.info(&format!("Found {} matching photos", photos.len())).await?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }
}
