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
        ctx.info("Fetching labels").await?;

        let labels = self.client
            .list_labels()
            .await
            .map_err(|e| McpError::internal(format!("Failed to list labels: {}", e)))?;

        ctx.info(&format!("Found {} labels", labels.len())).await?;

        let response = serde_json::json!({
            "labels": labels,
            "count": labels.len(),
            "note": "Limited to 50 labels."
        });

        Ok(serde_json::to_string_pretty(&response)?)
    }

    /// Get photos by label
    /// Returns photos that have a specific label/tag
    pub async fn get_photos_by_label(
        &self,
        ctx: Context,
        label: String,
        count: Option<u32>,
    ) -> McpResult<String> {
        // Default to 10, max 50
        let count = count.unwrap_or(10).min(50);

        if label.is_empty() {
            return Err(McpError::invalid_request("Label name cannot be empty"));
        }

        ctx.info(&format!("Fetching photos: label '{}'", label)).await?;

        let photos = self.client
            .get_photos_by_label(&label, count)
            .await
            .map_err(|e| McpError::internal(format!("Failed: {}", e)))?;

        ctx.info(&format!("Found {} photos", photos.len())).await?;

        let response = serde_json::json!({
            "photos": photos,
            "count": photos.len(),
            "label": label,
            "note": "Limited to 50 results max."
        });

        Ok(serde_json::to_string_pretty(&response)?)
    }
}
