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
        // Default to 10, max 50 to reduce token usage
        let count = count.unwrap_or(10).min(50);

        ctx.info(&format!("Searching photos: '{}'", query)).await?;

        let photos = self.client
            .search_photos(&query, count)
            .await
            .map_err(|e| McpError::internal(format!("Search failed: {}", e)))?;

        ctx.info(&format!("Found {} photos", photos.len())).await?;

        let response = serde_json::json!({
            "photos": photos,
            "count": photos.len(),
            "note": "Limited to 50 results max. Specify count parameter for different limit."
        });

        Ok(serde_json::to_string_pretty(&response)?)
    }
}
