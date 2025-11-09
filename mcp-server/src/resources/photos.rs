use crate::client::PhotoPrismClient;
use std::sync::Arc;
use turbomcp::prelude::*;

/// Photo resource handlers
pub struct PhotoResources {
    pub client: Arc<PhotoPrismClient>,
}

impl PhotoResources {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// Get recent photos
    /// URI: photoprism://photos/recent
    pub async fn recent_photos(&self) -> McpResult<String> {
        let photos = self
            .client
            .search_photos("", 20)
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch recent photos: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }

    /// Get photo by UID
    /// URI: photoprism://photos/{uid}
    pub async fn photo_by_uid(&self, uid: String) -> McpResult<String> {
        if uid.is_empty() {
            return Err(McpError::invalid_request("Photo UID cannot be empty"));
        }

        let photo = self
            .client
            .get_photo(&uid)
            .await
            .map_err(|e| McpError::internal(format!("Photo not found: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photo)?)
    }

    /// Get favorite photos
    /// URI: photoprism://photos/favorites
    pub async fn favorite_photos(&self) -> McpResult<String> {
        let photos = self
            .client
            .search_photos("favorite:true", 100)
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch favorites: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }

    /// Get photos by label
    /// URI: photoprism://photos/label/{label}
    pub async fn photos_by_label(&self, label: String) -> McpResult<String> {
        if label.is_empty() {
            return Err(McpError::invalid_request("Label cannot be empty"));
        }

        let photos = self
            .client
            .get_photos_by_label(&label, 100)
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch photos by label: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }
}
