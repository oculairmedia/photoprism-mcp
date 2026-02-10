use turbomcp::prelude::*;
use crate::client::PhotoPrismClient;
use crate::types::*;
use std::sync::Arc;

/// Album management tool handlers
pub struct AlbumTools {
    pub client: Arc<PhotoPrismClient>,
}

impl AlbumTools {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// List all albums
    pub async fn list_albums(&self, ctx: Context) -> McpResult<String> {
        ctx.info("Fetching all albums").await?;

        let albums = self.client
            .list_albums()
            .await
            .map_err(|e| McpError::internal(format!("Failed to list albums: {}", e)))?;

        ctx.info(&format!("Found {} albums", albums.len())).await?;

        let response = serde_json::json!({
            "albums": albums,
            "count": albums.len(),
            "note": "Limited to 20 albums. Use get_album with specific UID for details."
        });

        Ok(serde_json::to_string_pretty(&response)?)
    }

    /// Get album by UID
    pub async fn get_album(
        &self,
        ctx: Context,
        uid: String,
    ) -> McpResult<String> {
        if uid.is_empty() {
            return Err(McpError::invalid_request("Album UID cannot be empty"));
        }

        ctx.info(&format!("Fetching album: {}", uid)).await?;

        let album = self.client
            .get_album(&uid)
            .await
            .map_err(|e| McpError::internal(format!("Album not found: {}", e)))?;

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Create a new album
    pub async fn create_album(
        &self,
        ctx: Context,
        title: String,
        description: Option<String>,
    ) -> McpResult<String> {
        if title.is_empty() {
            return Err(McpError::invalid_request("Album title cannot be empty"));
        }

        ctx.info(&format!("Creating album: {}", title)).await?;

        let create = AlbumCreate {
            title: title.clone(),
            description,
            favorite: false,
        };

        let album = self.client
            .create_album(&create)
            .await
            .map_err(|e| McpError::internal(format!("Album creation failed: {}", e)))?;

        ctx.info(&format!("Album created with UID: {}", album.uid)).await?;

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Update album
    pub async fn update_album(
        &self,
        ctx: Context,
        uid: String,
        updates: AlbumUpdate,
    ) -> McpResult<String> {
        if uid.is_empty() {
            return Err(McpError::invalid_request("Album UID cannot be empty"));
        }

        ctx.info(&format!("Updating album: {}", uid)).await?;

        let album = self.client
            .update_album(&uid, &updates)
            .await
            .map_err(|e| McpError::internal(format!("Failed to update album: {}", e)))?;

        ctx.info(&format!("Album {} updated successfully", uid)).await?;

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Delete album
    pub async fn delete_album(
        &self,
        ctx: Context,
        uid: String,
    ) -> McpResult<String> {
        if uid.is_empty() {
            return Err(McpError::invalid_request("Album UID cannot be empty"));
        }

        ctx.warn(&format!("Deleting album: {}", uid)).await?;

        self.client
            .delete_album(&uid)
            .await
            .map_err(|e| McpError::internal(format!("Delete failed: {}", e)))?;

        Ok(format!("Album {} deleted successfully", uid))
    }

    /// Add photos to album
    pub async fn add_photos_to_album(
        &self,
        ctx: Context,
        album_uid: String,
        photo_uids: Vec<String>,
    ) -> McpResult<String> {
        if album_uid.is_empty() {
            return Err(McpError::invalid_request("Album UID cannot be empty"));
        }

        if photo_uids.is_empty() {
            return Err(McpError::invalid_request("Photo UIDs list cannot be empty"));
        }

        ctx.info(&format!(
            "Adding {} photos to album {}",
            photo_uids.len(),
            album_uid
        )).await?;

        self.client
            .add_photos_to_album(&album_uid, &photo_uids)
            .await
            .map_err(|e| McpError::internal(format!("Failed to add photos: {}", e)))?;

        Ok(format!(
            "Successfully added {} photos to album {}",
            photo_uids.len(),
            album_uid
        ))
    }
}
