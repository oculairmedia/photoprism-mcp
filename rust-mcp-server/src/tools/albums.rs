use crate::client::PhotoPrismClient;
use crate::types::{
    Album, AlbumCreateParams, AlbumSearchParams, AlbumUpdateParams,
    AddPhotosParams, RemovePhotosParams,
};
use std::sync::Arc;
use turbomcp::prelude::*;

/// Album tools for PhotoPrism MCP server
pub struct AlbumTools {
    client: Arc<PhotoPrismClient>,
}

impl AlbumTools {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// List all albums with optional filters
    #[tool("List albums from PhotoPrism library with optional search and filter criteria")]
    pub async fn list_albums(
        &self,
        ctx: Context,
        #[description("Search query for album title")] query: Option<String>,
        #[description("Filter by album type (album, folder, moment, month, state)")]
        album_type: Option<String>,
        #[description("Filter by category")] category: Option<String>,
        #[description("Show only favorites")] favorite: Option<bool>,
        #[description("Maximum number of results (default: 100, max: 1000)")]
        #[schemars(range(min = 1, max = 1000))]
        count: Option<u32>,
        #[description("Offset for pagination (default: 0)")] offset: Option<u32>,
    ) -> McpResult<String> {
        ctx.info("Listing albums...").await?;

        let params = AlbumSearchParams {
            q: query,
            album_type: album_type.and_then(|t| match t.to_lowercase().as_str() {
                "album" => Some(crate::types::AlbumType::Album),
                "folder" => Some(crate::types::AlbumType::Folder),
                "moment" => Some(crate::types::AlbumType::Moment),
                "month" => Some(crate::types::AlbumType::Month),
                "state" => Some(crate::types::AlbumType::State),
                _ => None,
            }),
            category,
            location: None,
            country: None,
            favorite,
            count: count.unwrap_or(100),
            offset: offset.unwrap_or(0),
            order: None,
        };

        let albums = self
            .client
            .list_albums(Some(params))
            .await
            .map_err(|e| McpError::internal(format!("Failed to list albums: {}", e)))?;

        ctx.info(&format!("Found {} albums", albums.len())).await?;

        Ok(serde_json::to_string_pretty(&albums)?)
    }

    /// Get detailed information about a specific album
    #[tool("Get detailed information about a specific album by UID")]
    pub async fn get_album(
        &self,
        ctx: Context,
        #[description("Album UID (16-character identifier)")]
        #[schemars(length(min = 16, max = 16))]
        uid: String,
    ) -> McpResult<String> {
        if uid.len() != 16 {
            return Err(McpError::invalid_request(
                "Album UID must be exactly 16 characters",
            ));
        }

        ctx.info(&format!("Fetching album: {}", uid)).await?;

        let album = self
            .client
            .get_album(&uid)
            .await
            .map_err(|e| McpError::not_found(format!("Album not found: {}", e)))?;

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Create a new album
    #[tool("Create a new album in PhotoPrism with title and optional metadata")]
    pub async fn create_album(
        &self,
        ctx: Context,
        #[description("Album title (required)")]
        #[schemars(length(min = 1, max = 255))]
        title: String,
        #[description("Album description")] description: Option<String>,
        #[description("Category")] category: Option<String>,
        #[description("Location")] location: Option<String>,
        #[description("Make this album a favorite")] favorite: Option<bool>,
        #[description("Make this album private")] private: Option<bool>,
    ) -> McpResult<String> {
        if title.trim().is_empty() {
            return Err(McpError::invalid_request("Album title cannot be empty"));
        }

        ctx.info(&format!("Creating album: {}", title)).await?;

        let params = AlbumCreateParams {
            title,
            description,
            category,
            location,
            favorite: favorite.unwrap_or(false),
            private: private.unwrap_or(false),
        };

        let album = self
            .client
            .create_album(params)
            .await
            .map_err(|e| McpError::internal(format!("Failed to create album: {}", e)))?;

        ctx.info(&format!("Album created with UID: {}", album.uid))
            .await?;

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Update an existing album's metadata
    #[tool("Update an existing album's title, description, or other metadata")]
    pub async fn update_album(
        &self,
        ctx: Context,
        #[description("Album UID to update")]
        #[schemars(length(min = 16, max = 16))]
        uid: String,
        #[description("New title")] title: Option<String>,
        #[description("New description")] description: Option<String>,
        #[description("New category")] category: Option<String>,
        #[description("New location")] location: Option<String>,
        #[description("Update favorite status")] favorite: Option<bool>,
        #[description("Update private status")] private: Option<bool>,
    ) -> McpResult<String> {
        if uid.len() != 16 {
            return Err(McpError::invalid_request(
                "Album UID must be exactly 16 characters",
            ));
        }

        ctx.info(&format!("Updating album: {}", uid)).await?;

        let params = AlbumUpdateParams {
            title,
            description,
            category,
            location,
            favorite,
            private,
            order: None,
        };

        let album = self
            .client
            .update_album(&uid, params)
            .await
            .map_err(|e| McpError::internal(format!("Failed to update album: {}", e)))?;

        ctx.info("Album updated successfully").await?;

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Delete an album
    #[tool("Delete an album by UID (WARNING: This action is permanent)")]
    pub async fn delete_album(
        &self,
        ctx: Context,
        #[description("Album UID to delete")]
        #[schemars(length(min = 16, max = 16))]
        uid: String,
        #[description("Confirm deletion (must be true to proceed)")] confirm: bool,
    ) -> McpResult<String> {
        if !confirm {
            return Err(McpError::invalid_request(
                "Deletion not confirmed. Set confirm=true to proceed.",
            ));
        }

        if uid.len() != 16 {
            return Err(McpError::invalid_request(
                "Album UID must be exactly 16 characters",
            ));
        }

        ctx.warn(&format!("Deleting album: {}", uid)).await?;

        self.client
            .delete_album(&uid)
            .await
            .map_err(|e| McpError::internal(format!("Failed to delete album: {}", e)))?;

        Ok(format!("Album {} deleted successfully", uid))
    }

    /// Add photos to an album
    #[tool("Add one or more photos to an album by their UIDs")]
    pub async fn add_photos_to_album(
        &self,
        ctx: Context,
        #[description("Album UID to add photos to")]
        #[schemars(length(min = 16, max = 16))]
        album_uid: String,
        #[description("List of photo UIDs to add (max 100)")]
        #[schemars(length(min = 1, max = 100))]
        photo_uids: Vec<String>,
    ) -> McpResult<String> {
        if album_uid.len() != 16 {
            return Err(McpError::invalid_request(
                "Album UID must be exactly 16 characters",
            ));
        }

        if photo_uids.is_empty() {
            return Err(McpError::invalid_request(
                "At least one photo UID must be provided",
            ));
        }

        if photo_uids.len() > 100 {
            return Err(McpError::invalid_request(
                "Maximum 100 photos can be added at once",
            ));
        }

        ctx.info(&format!(
            "Adding {} photos to album {}",
            photo_uids.len(),
            album_uid
        ))
        .await?;

        let album = self
            .client
            .add_photos_to_album(&album_uid, photo_uids)
            .await
            .map_err(|e| McpError::internal(format!("Failed to add photos: {}", e)))?;

        ctx.info("Photos added successfully").await?;

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Remove photos from an album
    #[tool("Remove one or more photos from an album by their UIDs")]
    pub async fn remove_photos_from_album(
        &self,
        ctx: Context,
        #[description("Album UID to remove photos from")]
        #[schemars(length(min = 16, max = 16))]
        album_uid: String,
        #[description("List of photo UIDs to remove (max 100)")]
        #[schemars(length(min = 1, max = 100))]
        photo_uids: Vec<String>,
    ) -> McpResult<String> {
        if album_uid.len() != 16 {
            return Err(McpError::invalid_request(
                "Album UID must be exactly 16 characters",
            ));
        }

        if photo_uids.is_empty() {
            return Err(McpError::invalid_request(
                "At least one photo UID must be provided",
            ));
        }

        if photo_uids.len() > 100 {
            return Err(McpError::invalid_request(
                "Maximum 100 photos can be removed at once",
            ));
        }

        ctx.info(&format!(
            "Removing {} photos from album {}",
            photo_uids.len(),
            album_uid
        ))
        .await?;

        self.client
            .remove_photos_from_album(&album_uid, photo_uids)
            .await
            .map_err(|e| McpError::internal(format!("Failed to remove photos: {}", e)))?;

        ctx.info("Photos removed successfully").await?;

        Ok(format!(
            "Successfully removed photos from album {}",
            album_uid
        ))
    }

    /// Toggle favorite status of an album
    #[tool("Mark an album as favorite or unfavorite")]
    pub async fn toggle_album_favorite(
        &self,
        ctx: Context,
        #[description("Album UID")]
        #[schemars(length(min = 16, max = 16))]
        uid: String,
        #[description("Set to true to favorite, false to unfavorite")] favorite: bool,
    ) -> McpResult<String> {
        if uid.len() != 16 {
            return Err(McpError::invalid_request(
                "Album UID must be exactly 16 characters",
            ));
        }

        ctx.info(&format!(
            "{} album: {}",
            if favorite { "Favoriting" } else { "Unfavoriting" },
            uid
        ))
        .await?;

        let album = if favorite {
            self.client
                .like_album(&uid)
                .await
                .map_err(|e| McpError::internal(format!("Failed to favorite album: {}", e)))?
        } else {
            self.client
                .unlike_album(&uid)
                .await
                .map_err(|e| McpError::internal(format!("Failed to unfavorite album: {}", e)))?
        };

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Clone an album (create a copy)
    #[tool("Create a copy of an existing album")]
    pub async fn clone_album(
        &self,
        ctx: Context,
        #[description("Album UID to clone")]
        #[schemars(length(min = 16, max = 16))]
        uid: String,
    ) -> McpResult<String> {
        if uid.len() != 16 {
            return Err(McpError::invalid_request(
                "Album UID must be exactly 16 characters",
            ));
        }

        ctx.info(&format!("Cloning album: {}", uid)).await?;

        let album = self
            .client
            .clone_album(&uid)
            .await
            .map_err(|e| McpError::internal(format!("Failed to clone album: {}", e)))?;

        ctx.info(&format!("Album cloned with new UID: {}", album.uid))
            .await?;

        Ok(serde_json::to_string_pretty(&album)?)
    }
}
