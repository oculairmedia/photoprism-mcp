use turbomcp::prelude::*;
use crate::client::PhotoPrismClient;
use std::sync::Arc;

/// Label tool handlers
pub struct LabelTools {
    pub client: Arc<PhotoPrismClient>,
}

impl LabelTools {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// List all labels
    /// Returns a list of all labels/tags in the PhotoPrism library
    pub async fn list_labels(&self, ctx: Context) -> McpResult<String> {
        ctx.info("Fetching all labels from PhotoPrism").await?;

        let labels = self.client
            .list_labels()
            .await
            .map_err(|e| McpError::internal(format!("Failed to list labels: {}", e)))?;

        ctx.info(&format!("Found {} labels", labels.len())).await?;

        Ok(serde_json::to_string_pretty(&labels)?)
    }

    /// Get photos by label
    /// Returns photos that have a specific label/tag
    pub async fn get_photos_by_label(
        &self,
        ctx: Context,
        label: String,
        count: Option<u32>,
    ) -> McpResult<String> {
        let count = count.unwrap_or(100);

        if label.is_empty() {
            return Err(McpError::invalid_request("Label name cannot be empty"));
        }

        ctx.info(&format!("Fetching photos with label '{}'", label)).await?;

        let photos = self.client
            .get_photos_by_label(&label, count)
            .await
            .map_err(|e| McpError::internal(format!("Failed to get photos by label: {}", e)))?;

        ctx.info(&format!("Found {} photos with label '{}'", photos.len(), label)).await?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }
}
