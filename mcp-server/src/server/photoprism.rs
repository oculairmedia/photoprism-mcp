use std::sync::Arc;
use turbomcp::prelude::*;
use crate::client::PhotoPrismClient;
use crate::config::Config;
use crate::types::*;

/// Main PhotoPrism MCP Server
#[derive(Clone)]
pub struct PhotoPrismServer {
    /// PhotoPrism API client
    client: Arc<PhotoPrismClient>,
}

#[turbomcp::server(
    name = "photoprism",
    version = "0.1.0"
)]
impl PhotoPrismServer {
    /// Create a new PhotoPrism MCP server
    pub fn new(config: Config) -> Result<Self, anyhow::Error> {
        let client = PhotoPrismClient::new(
            config.base_url.clone(),
            config.username.clone(),
            config.password.clone(),
        )?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    // Photo tools

    /// Get photo details by UID
    #[tool("Get photo details by UID")]
    async fn get_photo(
        &self,
        ctx: Context,
        uid: String,
    ) -> McpResult<String> {
        if uid.is_empty() || uid.len() != 16 {
            return Err(McpError::invalid_request("Invalid photo UID"));
        }

        ctx.info(&format!("Fetching photo: {}", uid)).await?;

        let photo = self.client
            .get_photo(&uid)
            .await
            .map_err(|e| McpError::internal(format!("Photo not found: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photo)?)
    }

    /// Update photo metadata
    #[tool("Update photo metadata")]
    async fn update_photo(
        &self,
        ctx: Context,
        uid: String,
        title: Option<String>,
        description: Option<String>,
        favorite: Option<bool>,
    ) -> McpResult<String> {
        if uid.is_empty() {
            return Err(McpError::invalid_request("Photo UID cannot be empty"));
        }

        ctx.info(&format!("Updating photo: {}", uid)).await?;

        let updates = PhotoUpdate {
            title,
            description,
            favorite,
            private: None,
        };

        let photo = self.client
            .update_photo(&uid, &updates)
            .await
            .map_err(|e| McpError::internal(format!("Update failed: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photo)?)
    }

    /// Search photos by text query
    #[tool("Search photos by text query")]
    async fn search_photos(
        &self,
        ctx: Context,
        query: String,
        count: Option<u32>,
    ) -> McpResult<String> {
        let count = count.unwrap_or(100).min(1000);

        ctx.info(&format!("Searching photos: '{}'", query)).await?;

        let photos = self.client
            .search_photos(&query, count)
            .await
            .map_err(|e| McpError::internal(format!("Search failed: {}", e)))?;

        ctx.info(&format!("Found {} photos", photos.len())).await?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }

    // Album tools

    /// List all albums
    #[tool("List all albums")]
    async fn list_albums(&self, ctx: Context) -> McpResult<String> {
        ctx.info("Fetching all albums").await?;

        let albums = self.client
            .list_albums()
            .await
            .map_err(|e| McpError::internal(format!("Failed to list albums: {}", e)))?;

        ctx.info(&format!("Found {} albums", albums.len())).await?;

        Ok(serde_json::to_string_pretty(&albums)?)
    }

    /// Get album details by UID
    #[tool("Get album details by UID")]
    async fn get_album(
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
    #[tool("Create a new album")]
    async fn create_album(
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
            title,
            description,
            favorite: false,
        };

        let album = self.client
            .create_album(&create)
            .await
            .map_err(|e| McpError::internal(format!("Album creation failed: {}", e)))?;

        ctx.info(&format!("Album created: {}", album.uid)).await?;

        Ok(serde_json::to_string_pretty(&album)?)
    }

    /// Add photos to an album
    #[tool("Add photos to an album")]
    async fn add_photos_to_album(
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

    // Label tools

    /// List all labels/tags
    #[tool("List all labels/tags")]
    async fn list_labels(&self, ctx: Context) -> McpResult<String> {
        ctx.info("Fetching all labels").await?;

        let labels = self.client
            .list_labels()
            .await
            .map_err(|e| McpError::internal(format!("Failed to list labels: {}", e)))?;

        ctx.info(&format!("Found {} labels", labels.len())).await?;

        Ok(serde_json::to_string_pretty(&labels)?)
    }

    /// Get photos with a specific label
    #[tool("Get photos with a specific label")]
    async fn get_photos_by_label(
        &self,
        ctx: Context,
        label: String,
        count: Option<u32>,
    ) -> McpResult<String> {
        let count = count.unwrap_or(100);

        if label.is_empty() {
            return Err(McpError::invalid_request("Label name cannot be empty"));
        }

        ctx.info(&format!("Fetching photos with label: '{}'", label)).await?;

        let photos = self.client
            .get_photos_by_label(&label, count)
            .await
            .map_err(|e| McpError::internal(format!("Failed to get photos: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }

    // Subject tools

    /// List all subjects/people
    #[tool("List all subjects/people")]
    async fn list_subjects(&self, ctx: Context) -> McpResult<String> {
        ctx.info("Fetching all subjects").await?;

        let subjects = self.client
            .list_subjects()
            .await
            .map_err(|e| McpError::internal(format!("Failed to list subjects: {}", e)))?;

        ctx.info(&format!("Found {} subjects", subjects.len())).await?;

        Ok(serde_json::to_string_pretty(&subjects)?)
    }

    /// Get photos of a specific subject/person
    #[tool("Get photos of a specific subject/person")]
    async fn get_subject_photos(
        &self,
        ctx: Context,
        subject_uid: String,
        count: Option<u32>,
    ) -> McpResult<String> {
        let count = count.unwrap_or(100);

        if subject_uid.is_empty() {
            return Err(McpError::invalid_request("Subject UID cannot be empty"));
        }

        ctx.info(&format!("Fetching photos for subject: {}", subject_uid)).await?;

        let photos = self.client
            .get_photos_by_subject(&subject_uid, count)
            .await
            .map_err(|e| McpError::internal(format!("Failed to get photos: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }

    // Library tools

    /// Get PhotoPrism library status
    #[tool("Get PhotoPrism library status")]
    async fn get_status(&self, ctx: Context) -> McpResult<String> {
        ctx.info("Fetching library status").await?;

        let status = self.client
            .get_status()
            .await
            .map_err(|e| McpError::internal(format!("Failed to get status: {}", e)))?;

        Ok(serde_json::to_string_pretty(&status)?)
    }

    // Batch operation tools

    /// Archive or restore multiple photos in batch
    #[tool("Archive or restore multiple photos in batch (max 100 photos per operation)")]
    async fn batch_archive_photos(
        &self,
        ctx: Context,
        photo_uids: Vec<String>,
        restore: Option<bool>,
    ) -> McpResult<String> {
        crate::tools::batch::batch_archive_photos(&self.client, ctx, photo_uids, restore).await
    }

    /// Delete multiple photos in batch
    #[tool("Delete multiple photos in batch - PERMANENT operation with safety confirmation required (max 50 photos)")]
    async fn batch_delete_photos(
        &self,
        ctx: Context,
        photo_uids: Vec<String>,
        confirm: bool,
        permanent: Option<bool>,
    ) -> McpResult<String> {
        crate::tools::batch::batch_delete_photos(&self.client, ctx, photo_uids, confirm, permanent).await
    }

    /// Mark multiple photos as favorite or unfavorite
    #[tool("Mark multiple photos as favorite or unfavorite in batch (max 100 photos)")]
    async fn batch_favorite_photos(
        &self,
        ctx: Context,
        photo_uids: Vec<String>,
        unfavorite: Option<bool>,
    ) -> McpResult<String> {
        crate::tools::batch::batch_favorite_photos(&self.client, ctx, photo_uids, unfavorite).await
    }

    /// Make multiple photos private or public
    #[tool("Make multiple photos private or public in batch (max 100 photos)")]
    async fn batch_private_photos(
        &self,
        ctx: Context,
        photo_uids: Vec<String>,
        make_public: Option<bool>,
    ) -> McpResult<String> {
        crate::tools::batch::batch_private_photos(&self.client, ctx, photo_uids, make_public).await
    }

    /// Update metadata for multiple photos in batch
    #[tool("Update title, description, or tags for multiple photos in batch (max 50 photos)")]
    async fn batch_update_photos(
        &self,
        ctx: Context,
        photo_uids: Vec<String>,
        title: Option<String>,
        description: Option<String>,
        add_tags: Option<Vec<String>>,
    ) -> McpResult<String> {
        crate::tools::batch::batch_update_photos(&self.client, ctx, photo_uids, title, description, add_tags).await
    }

    // Resources

    /// Recent photos
    #[resource("photoprism://photos/recent")]
    async fn recent_photos(&self) -> McpResult<String> {
        let photos = self.client
            .search_photos("", 20)
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch recent photos: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }

    /// Favorite photos
    #[resource("photoprism://photos/favorites")]
    async fn favorite_photos(&self) -> McpResult<String> {
        let photos = self.client
            .search_photos("favorite:true", 100)
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch favorites: {}", e)))?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }

    /// All albums
    #[resource("photoprism://albums/list")]
    async fn albums_list(&self) -> McpResult<String> {
        let albums = self.client
            .list_albums()
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch albums: {}", e)))?;

        Ok(serde_json::to_string_pretty(&albums)?)
    }

    /// System status
    #[resource("photoprism://system/status")]
    async fn system_status(&self) -> McpResult<String> {
        let status = self.client
            .get_status()
            .await
            .map_err(|e| McpError::internal(format!("Failed to fetch status: {}", e)))?;

        Ok(serde_json::to_string_pretty(&status)?)
    }
}
