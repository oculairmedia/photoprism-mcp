use turbomcp::prelude::*;
use crate::client::PhotoPrismClient;
use std::sync::Arc;

/// Subject (people) tool handlers
pub struct SubjectTools {
    pub client: Arc<PhotoPrismClient>,
}

impl SubjectTools {
    pub fn new(client: Arc<PhotoPrismClient>) -> Self {
        Self { client }
    }

    /// List all subjects
    /// Returns a list of all people/subjects recognized in the PhotoPrism library
    pub async fn list_subjects(&self, ctx: Context) -> McpResult<String> {
        ctx.info("Fetching all subjects from PhotoPrism").await?;

        let subjects = self.client
            .list_subjects()
            .await
            .map_err(|e| {
                ctx.error(&format!("Failed to list subjects: {}", e));
                McpError::internal(format!("Failed to list subjects: {}", e))
            })?;

        ctx.info(&format!("Found {} subjects", subjects.len())).await?;

        Ok(serde_json::to_string_pretty(&subjects)?)
    }

    /// Get photos by subject
    /// Returns photos containing a specific person/subject
    pub async fn get_subject_photos(
        &self,
        ctx: Context,
        subject_uid: String,
        count: Option<u32>,
    ) -> McpResult<String> {
        let count = count.unwrap_or(100);

        if subject_uid.is_empty() {
            return Err(McpError::invalid_request("Subject UID cannot be empty"));
        }

        ctx.info(&format!("Fetching photos for subject '{}'", subject_uid)).await?;

        let photos = self.client
            .get_photos_by_subject(&subject_uid, count)
            .await
            .map_err(|e| {
                ctx.error(&format!("Failed to get photos by subject: {}", e));
                McpError::internal(format!("Failed to get photos by subject: {}", e))
            })?;

        ctx.info(&format!("Found {} photos with subject '{}'", photos.len(), subject_uid)).await?;

        Ok(serde_json::to_string_pretty(&photos)?)
    }
}
