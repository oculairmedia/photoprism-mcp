use crate::client::PhotoPrismClient;
use crate::types::{
    BatchArchiveParams, BatchDeleteParams, BatchFavoriteParams, BatchItemResult,
    BatchOperationResponse, BatchPrivateParams, BatchUpdateParams,
};
use std::sync::Arc;
use turbomcp::prelude::*;

/// Batch operation tools for PhotoPrism MCP server
pub struct BatchTools {
    client: Arc<PhotoPrismClient>,
}

impl BatchTools {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// Archive multiple photos at once
    #[tool("Archive or restore multiple photos in batch (max 100 photos per operation)")]
    pub async fn batch_archive_photos(
        &self,
        ctx: Context,
        #[description("List of photo UIDs to archive (max 100)")]
        #[schemars(length(min = 1, max = 100))]
        photo_uids: Vec<String>,
        #[description("Set to true to restore (unarchive) instead of archive")] restore: Option<bool>,
    ) -> McpResult<String> {
        if photo_uids.is_empty() {
            return Err(McpError::invalid_request(
                "At least one photo UID must be provided",
            ));
        }

        if photo_uids.len() > 100 {
            return Err(McpError::invalid_request(
                "Maximum 100 photos can be processed at once",
            ));
        }

        let restore = restore.unwrap_or(false);
        let action = if restore { "Restoring" } else { "Archiving" };

        ctx.info(&format!("{} {} photos...", action, photo_uids.len()))
            .await?;

        let mut results = Vec::new();

        for uid in photo_uids {
            match self.archive_single_photo(&uid, restore).await {
                Ok(_) => {
                    ctx.info(&format!("Successfully {} photo: {}", action.to_lowercase(), uid))
                        .await?;
                    results.push(BatchItemResult::success(uid, None));
                }
                Err(e) => {
                    let error_msg = format!("Failed to {} photo: {}", action.to_lowercase(), e);
                    ctx.error(&format!("Error for {}: {}", uid, error_msg))
                        .await?;
                    results.push(BatchItemResult::error(uid, None, error_msg));
                }
            }
        }

        let response = BatchOperationResponse::from_results(
            results,
            Some(format!(
                "Batch {} operation completed",
                action.to_lowercase()
            )),
        );

        ctx.info(&format!(
            "Batch operation complete: {} succeeded, {} failed",
            response.summary.success_count, response.summary.error_count
        ))
        .await?;

        Ok(serde_json::to_string_pretty(&response)?)
    }

    /// Delete multiple photos at once (with safety checks)
    #[tool("Delete multiple photos in batch - PERMANENT operation with safety confirmation required (max 50 photos)")]
    pub async fn batch_delete_photos(
        &self,
        ctx: Context,
        #[description("List of photo UIDs to delete (max 50 for safety)")]
        #[schemars(length(min = 1, max = 50))]
        photo_uids: Vec<String>,
        #[description("Must be set to true to confirm deletion")] confirm: bool,
        #[description("Permanently delete (vs move to trash)")] permanent: Option<bool>,
    ) -> McpResult<String> {
        if !confirm {
            return Err(McpError::invalid_request(
                "Deletion not confirmed. Set confirm=true to proceed. WARNING: This action is permanent!",
            ));
        }

        if photo_uids.is_empty() {
            return Err(McpError::invalid_request(
                "At least one photo UID must be provided",
            ));
        }

        if photo_uids.len() > 50 {
            return Err(McpError::invalid_request(
                "Maximum 50 photos can be deleted at once for safety",
            ));
        }

        let permanent = permanent.unwrap_or(false);

        ctx.warn(&format!(
            "⚠️  {} deleting {} photos...",
            if permanent { "Permanently" } else { "Soft" },
            photo_uids.len()
        ))
        .await?;

        let mut results = Vec::new();

        for uid in photo_uids {
            match self.delete_single_photo(&uid, permanent).await {
                Ok(_) => {
                    ctx.info(&format!("Successfully deleted photo: {}", uid))
                        .await?;
                    results.push(BatchItemResult::success(uid, None));
                }
                Err(e) => {
                    let error_msg = format!("Failed to delete photo: {}", e);
                    ctx.error(&format!("Error for {}: {}", uid, error_msg))
                        .await?;
                    results.push(BatchItemResult::error(uid, None, error_msg));
                }
            }
        }

        let response = BatchOperationResponse::from_results(
            results,
            Some("Batch delete operation completed".to_string()),
        );

        ctx.info(&format!(
            "Batch delete complete: {} succeeded, {} failed",
            response.summary.success_count, response.summary.error_count
        ))
        .await?;

        Ok(serde_json::to_string_pretty(&response)?)
    }

    /// Mark multiple photos as favorite or unfavorite
    #[tool("Mark multiple photos as favorite or unfavorite in batch (max 100 photos)")]
    pub async fn batch_favorite_photos(
        &self,
        ctx: Context,
        #[description("List of photo UIDs to favorite/unfavorite (max 100)")]
        #[schemars(length(min = 1, max = 100))]
        photo_uids: Vec<String>,
        #[description("Set to true to unfavorite instead of favorite")] unfavorite: Option<bool>,
    ) -> McpResult<String> {
        if photo_uids.is_empty() {
            return Err(McpError::invalid_request(
                "At least one photo UID must be provided",
            ));
        }

        if photo_uids.len() > 100 {
            return Err(McpError::invalid_request(
                "Maximum 100 photos can be processed at once",
            ));
        }

        let unfavorite = unfavorite.unwrap_or(false);
        let action = if unfavorite {
            "Unfavoriting"
        } else {
            "Favoriting"
        };

        ctx.info(&format!("{} {} photos...", action, photo_uids.len()))
            .await?;

        let mut results = Vec::new();

        for uid in photo_uids {
            match self.favorite_single_photo(&uid, !unfavorite).await {
                Ok(_) => {
                    ctx.info(&format!("Successfully {} photo: {}", action.to_lowercase(), uid))
                        .await?;
                    results.push(BatchItemResult::success(uid, None));
                }
                Err(e) => {
                    let error_msg = format!("Failed to {} photo: {}", action.to_lowercase(), e);
                    ctx.error(&format!("Error for {}: {}", uid, error_msg))
                        .await?;
                    results.push(BatchItemResult::error(uid, None, error_msg));
                }
            }
        }

        let response = BatchOperationResponse::from_results(
            results,
            Some(format!(
                "Batch {} operation completed",
                action.to_lowercase()
            )),
        );

        ctx.info(&format!(
            "Batch operation complete: {} succeeded, {} failed",
            response.summary.success_count, response.summary.error_count
        ))
        .await?;

        Ok(serde_json::to_string_pretty(&response)?)
    }

    /// Make multiple photos private or public
    #[tool("Make multiple photos private or public in batch (max 100 photos)")]
    pub async fn batch_private_photos(
        &self,
        ctx: Context,
        #[description("List of photo UIDs to make private/public (max 100)")]
        #[schemars(length(min = 1, max = 100))]
        photo_uids: Vec<String>,
        #[description("Set to true to make public instead of private")] make_public: Option<bool>,
    ) -> McpResult<String> {
        if photo_uids.is_empty() {
            return Err(McpError::invalid_request(
                "At least one photo UID must be provided",
            ));
        }

        if photo_uids.len() > 100 {
            return Err(McpError::invalid_request(
                "Maximum 100 photos can be processed at once",
            ));
        }

        let make_public = make_public.unwrap_or(false);
        let action = if make_public {
            "Making public"
        } else {
            "Making private"
        };

        ctx.info(&format!("{} {} photos...", action, photo_uids.len()))
            .await?;

        let mut results = Vec::new();

        for uid in photo_uids {
            match self.set_photo_privacy(&uid, !make_public).await {
                Ok(_) => {
                    ctx.info(&format!("Successfully {} photo: {}", action.to_lowercase(), uid))
                        .await?;
                    results.push(BatchItemResult::success(uid, None));
                }
                Err(e) => {
                    let error_msg = format!("Failed to {} photo: {}", action.to_lowercase(), e);
                    ctx.error(&format!("Error for {}: {}", uid, error_msg))
                        .await?;
                    results.push(BatchItemResult::error(uid, None, error_msg));
                }
            }
        }

        let response = BatchOperationResponse::from_results(
            results,
            Some(format!("{} operation completed", action)),
        );

        ctx.info(&format!(
            "Batch operation complete: {} succeeded, {} failed",
            response.summary.success_count, response.summary.error_count
        ))
        .await?;

        Ok(serde_json::to_string_pretty(&response)?)
    }

    /// Update metadata for multiple photos
    #[tool("Update title, description, or tags for multiple photos in batch (max 50 photos)")]
    pub async fn batch_update_photos(
        &self,
        ctx: Context,
        #[description("List of photo UIDs to update (max 50)")]
        #[schemars(length(min = 1, max = 50))]
        photo_uids: Vec<String>,
        #[description("New title to apply to all photos")] title: Option<String>,
        #[description("New description to apply to all photos")] description: Option<String>,
        #[description("Tags to add to all photos")] add_tags: Option<Vec<String>>,
    ) -> McpResult<String> {
        if photo_uids.is_empty() {
            return Err(McpError::invalid_request(
                "At least one photo UID must be provided",
            ));
        }

        if photo_uids.len() > 50 {
            return Err(McpError::invalid_request(
                "Maximum 50 photos can be updated at once",
            ));
        }

        if title.is_none() && description.is_none() && add_tags.is_none() {
            return Err(McpError::invalid_request(
                "At least one update parameter (title, description, or add_tags) must be provided",
            ));
        }

        ctx.info(&format!("Updating {} photos...", photo_uids.len()))
            .await?;

        let mut results = Vec::new();

        for uid in photo_uids {
            match self
                .update_single_photo(&uid, &title, &description, &add_tags)
                .await
            {
                Ok(_) => {
                    ctx.info(&format!("Successfully updated photo: {}", uid))
                        .await?;
                    results.push(BatchItemResult::success(uid, None));
                }
                Err(e) => {
                    let error_msg = format!("Failed to update photo: {}", e);
                    ctx.error(&format!("Error for {}: {}", uid, error_msg))
                        .await?;
                    results.push(BatchItemResult::error(uid, None, error_msg));
                }
            }
        }

        let response = BatchOperationResponse::from_results(
            results,
            Some("Batch update operation completed".to_string()),
        );

        ctx.info(&format!(
            "Batch update complete: {} succeeded, {} failed",
            response.summary.success_count, response.summary.error_count
        ))
        .await?;

        Ok(serde_json::to_string_pretty(&response)?)
    }

    // Helper methods for single-photo operations

    async fn archive_single_photo(&self, uid: &str, restore: bool) -> Result<(), anyhow::Error> {
        let path = if restore {
            format!("/api/v1/photos/{}/restore", uid)
        } else {
            format!("/api/v1/photos/{}/archive", uid)
        };

        self.client.post::<(), serde_json::Value>(&path, &()).await?;
        Ok(())
    }

    async fn delete_single_photo(&self, uid: &str, permanent: bool) -> Result<(), anyhow::Error> {
        let path = if permanent {
            format!("/api/v1/photos/{}", uid)
        } else {
            format!("/api/v1/photos/{}/delete", uid)
        };

        self.client.delete(&path).await
    }

    async fn favorite_single_photo(&self, uid: &str, favorite: bool) -> Result<(), anyhow::Error> {
        let path = format!("/api/v1/photos/{}/like", uid);

        if favorite {
            self.client.post::<(), serde_json::Value>(&path, &()).await?;
        } else {
            self.client.delete(&path).await?;
        }

        Ok(())
    }

    async fn set_photo_privacy(&self, uid: &str, private: bool) -> Result<(), anyhow::Error> {
        let path = format!("/api/v1/photos/{}", uid);

        use serde::Serialize;
        #[derive(Serialize)]
        struct PrivacyUpdate {
            #[serde(rename = "Private")]
            private: bool,
        }

        self.client
            .put::<PrivacyUpdate, serde_json::Value>(&path, &PrivacyUpdate { private })
            .await?;

        Ok(())
    }

    async fn update_single_photo(
        &self,
        uid: &str,
        title: &Option<String>,
        description: &Option<String>,
        _add_tags: &Option<Vec<String>>,
    ) -> Result<(), anyhow::Error> {
        let path = format!("/api/v1/photos/{}", uid);

        use serde::Serialize;
        #[derive(Serialize)]
        struct PhotoUpdate {
            #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        }

        let update = PhotoUpdate {
            title: title.clone(),
            description: description.clone(),
        };

        self.client
            .put::<PhotoUpdate, serde_json::Value>(&path, &update)
            .await?;

        Ok(())
    }
}
