use crate::client::PhotoPrismClient;
use std::sync::Arc;
use turbomcp::prelude::*;

/// Album resource handlers
pub struct AlbumResources {
    pub client: Arc<PhotoPrismClient>,
}

impl AlbumResources {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// List all albums
    /// URI: photoprism://albums/list
    pub async fn list_albums(&self) -> McpResult<String> {
        let albums = self
            .client
            .list_albums()
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch albums: {}", e)))?;

        Ok(serde_json::to_string_pretty(&albums)?)
    }

    /// Get album by UID
    /// URI: photoprism://albums/{uid}
    pub async fn album_by_uid(&self, uid: String) -> McpResult<String> {
        if uid.is_empty() {
            return Err(McpError::invalid_request("Album UID cannot be empty"));
        }

        let album = self
            .client
            .get_album(&uid)
            .await
            .map_err(|e| McpError::internal(format!("Album not found: {}", e)))?;

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Get favorite albums
    /// URI: photoprism://albums/favorites
    pub async fn favorite_albums(&self) -> McpResult<String> {
        let all_albums = self
            .client
            .list_albums()
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch albums: {}", e)))?;

        let favorites: Vec<_> = all_albums.into_iter().filter(|a| a.favorite).collect();

        Ok(serde_json::to_string_pretty(&favorites)?)
    }
}
