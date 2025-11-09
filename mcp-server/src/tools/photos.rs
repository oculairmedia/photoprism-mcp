use crate::client::PhotoPrismClient;
use crate::types::*;
use std::sync::Arc;
use turbomcp::prelude::*;

/// Photo management tool handlers
pub struct PhotoTools {
    pub client: Arc<PhotoPrismClient>,
}

impl PhotoTools {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// Get photo by UID
    pub async fn get_photo(&self, ctx: Context, uid: String) -> McpResult<String> {
        if uid.is_empty() {
            return Err(McpError::invalid_request("Photo UID cannot be empty"));
        }

        if uid.len() != 16 {
            return Err(McpError::invalid_request(
                "Invalid UID format (must be 16 characters)",
            ));
        }

        ctx.info(&format!("Fetching photo: {}", uid)).await?;

        let photo = self
            .client
            .get_photo(&uid)
            .await
            .map_err(|e| McpError::internal(format!("Photo not found: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photo)?)
    }

    /// Update photo metadata
    pub async fn update_photo(
        &self,
        ctx: Context,
        uid: String,
        updates: PhotoUpdate,
    ) -> McpResult<String> {
        if uid.is_empty() {
            return Err(McpError::invalid_request("Photo UID cannot be empty"));
        }

        ctx.info(&format!("Updating photo: {}", uid)).await?;

        let photo = self
            .client
            .update_photo(&uid, &updates)
            .await
            .map_err(|e| McpError::internal(format!("Failed to update photo: {}", e)))?;

        ctx.info(&format!("Photo {} updated successfully", uid))
            .await?;

        Ok(serde_json::to_string_pretty(&photo)?)
    }

    /// Delete photo (requires confirmation)
    pub async fn delete_photo(&self, ctx: Context, uid: String) -> McpResult<String> {
        if uid.is_empty() {
            return Err(McpError::invalid_request("Photo UID cannot be empty"));
        }

        ctx.warn(&format!("Deleting photo: {}", uid)).await?;

        self.client
            .delete_photo(&uid)
            .await
            .map_err(|e| McpError::internal(format!("Delete failed: {}", e)))?;

        Ok(format!("Photo {} deleted successfully", uid))
    }
}
