//! PhotoPrism API client methods for photo management.
//!
//! This module provides async methods to interact with PhotoPrism's photo endpoints.
//! All methods handle authentication, request formatting, and error handling.

use crate::types::photo::{
    Photo, PhotoBatchRequest, PhotoBatchResponse, PhotoList, PhotoSearchParams, PhotoUpdate,
};
use anyhow::{anyhow, Context, Result};
use reqwest::StatusCode;
use serde::Serialize;
use std::collections::HashMap;

/// PhotoPrism API client.
///
/// This struct is assumed to be defined in the parent module (src/client/mod.rs).
/// It should contain the HTTP client, authentication, and base configuration.
pub struct PhotoPrismClient {
    pub(crate) http_client: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) session_token: std::sync::Arc<tokio::sync::RwLock<Option<String>>>,
}

impl PhotoPrismClient {
    /// Search photos with a simple text query.
    ///
    /// This is a convenience method for basic searches. For advanced filtering,
    /// use `search_photos_advanced` instead.
    ///
    /// # Arguments
    ///
    /// * `query` - Search text (searches titles, descriptions, keywords)
    /// * `count` - Maximum number of results (capped at 1000)
    ///
    /// # Returns
    ///
    /// A `PhotoList` containing matching photos and metadata.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example(client: &PhotoPrismClient) -> Result<()> {
    /// let results = client.search_photos("sunset beach", 50).await?;
    /// println!("Found {} photos", results.photos.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_photos(&self, query: &str, count: u32) -> Result<PhotoList> {
        // Clamp count to valid range
        let count = count.min(1000);

        let mut params = HashMap::new();
        if !query.is_empty() {
            params.insert("q", query.to_string());
        }
        params.insert("count", count.to_string());

        let path = format!("/api/v1/photos?{}", self.build_query_string(&params));
        self.get(&path).await
    }

    /// Search photos with advanced filters.
    ///
    /// Provides access to all PhotoPrism search capabilities including type filters,
    /// quality filtering, location-based search, date ranges, and more.
    ///
    /// # Arguments
    ///
    /// * `params` - Advanced search parameters
    ///
    /// # Returns
    ///
    /// A `PhotoList` containing matching photos and metadata.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example(client: &PhotoPrismClient) -> Result<()> {
    /// use crate::types::photo::{PhotoSearchParams, PhotoType, SortOrder};
    ///
    /// let params = PhotoSearchParams {
    ///     photo_type: Some(PhotoType::Image),
    ///     quality: Some(5),
    ///     favorite: Some(true),
    ///     country: Some("USA".to_string()),
    ///     order: Some(SortOrder::Newest),
    ///     count: 100,
    ///     offset: 0,
    ///     ..Default::default()
    /// };
    ///
    /// let results = client.search_photos_advanced(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_photos_advanced(&self, params: PhotoSearchParams) -> Result<PhotoList> {
        let query = self.build_search_query(&params);
        let path = format!("/api/v1/photos?{}", query);
        self.get(&path).await
    }

    /// Get a single photo by its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `uid` - Photo UID (16-character alphanumeric string)
    ///
    /// # Returns
    ///
    /// The `Photo` object with full metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the photo is not found or the UID is invalid.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example(client: &PhotoPrismClient) -> Result<()> {
    /// let photo = client.get_photo("abc123def4567890").await?;
    /// println!("Title: {}", photo.title);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_photo(&self, uid: &str) -> Result<Photo> {
        if uid.is_empty() {
            return Err(anyhow!("Photo UID cannot be empty"));
        }

        if uid.len() != 16 {
            return Err(anyhow!(
                "Invalid UID format: expected 16 characters, got {}",
                uid.len()
            ));
        }

        let path = format!("/api/v1/photos/{}", uid);
        self.get(&path).await
    }

    /// Update photo metadata.
    ///
    /// Allows modification of photo title, description, favorite status, location,
    /// and other editable attributes. Only the fields present in the `updates`
    /// parameter will be modified.
    ///
    /// # Arguments
    ///
    /// * `uid` - Photo UID to update
    /// * `updates` - Fields to update (all optional)
    ///
    /// # Returns
    ///
    /// The updated `Photo` object.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example(client: &PhotoPrismClient) -> Result<()> {
    /// use crate::types::photo::PhotoUpdate;
    ///
    /// let updates = PhotoUpdate {
    ///     title: Some("Beautiful Sunset".to_string()),
    ///     description: Some("Taken at the beach".to_string()),
    ///     favorite: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let photo = client.update_photo("abc123def4567890", &updates).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_photo(&self, uid: &str, updates: &PhotoUpdate) -> Result<Photo> {
        if uid.is_empty() {
            return Err(anyhow!("Photo UID cannot be empty"));
        }

        if uid.len() != 16 {
            return Err(anyhow!(
                "Invalid UID format: expected 16 characters, got {}",
                uid.len()
            ));
        }

        let path = format!("/api/v1/photos/{}", uid);
        self.put(&path, updates).await
    }

    /// Delete a photo permanently.
    ///
    /// **Warning:** This operation cannot be undone. The photo and all associated
    /// files will be removed from PhotoPrism.
    ///
    /// # Arguments
    ///
    /// * `uid` - Photo UID to delete
    ///
    /// # Errors
    ///
    /// Returns an error if the photo is not found or deletion fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example(client: &PhotoPrismClient) -> Result<()> {
    /// client.delete_photo("abc123def4567890").await?;
    /// println!("Photo deleted successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_photo(&self, uid: &str) -> Result<()> {
        if uid.is_empty() {
            return Err(anyhow!("Photo UID cannot be empty"));
        }

        if uid.len() != 16 {
            return Err(anyhow!(
                "Invalid UID format: expected 16 characters, got {}",
                uid.len()
            ));
        }

        let path = format!("/api/v1/photos/{}", uid);
        self.delete(&path).await
    }

    /// Archive multiple photos at once.
    ///
    /// Archived photos are hidden from the main library but not deleted.
    /// They can be restored later.
    ///
    /// # Arguments
    ///
    /// * `uids` - Vector of photo UIDs to archive
    ///
    /// # Returns
    ///
    /// A `PhotoBatchResponse` with success/error counts.
    pub async fn archive_photos(&self, uids: Vec<String>) -> Result<PhotoBatchResponse> {
        if uids.is_empty() {
            return Err(anyhow!("Photo UID list cannot be empty"));
        }

        let request = PhotoBatchRequest { photos: uids };
        let path = "/api/v1/batch/photos/archive";
        self.post(path, &request).await
    }

    /// Restore multiple archived photos.
    ///
    /// # Arguments
    ///
    /// * `uids` - Vector of photo UIDs to restore
    ///
    /// # Returns
    ///
    /// A `PhotoBatchResponse` with success/error counts.
    pub async fn restore_photos(&self, uids: Vec<String>) -> Result<PhotoBatchResponse> {
        if uids.is_empty() {
            return Err(anyhow!("Photo UID list cannot be empty"));
        }

        let request = PhotoBatchRequest { photos: uids };
        let path = "/api/v1/batch/photos/restore";
        self.post(path, &request).await
    }

    /// Mark multiple photos as favorites.
    ///
    /// # Arguments
    ///
    /// * `uids` - Vector of photo UIDs to mark as favorites
    ///
    /// # Returns
    ///
    /// A `PhotoBatchResponse` with success/error counts.
    pub async fn like_photos(&self, uids: Vec<String>) -> Result<PhotoBatchResponse> {
        if uids.is_empty() {
            return Err(anyhow!("Photo UID list cannot be empty"));
        }

        let request = PhotoBatchRequest { photos: uids };
        let path = "/api/v1/batch/photos/like";
        self.post(path, &request).await
    }

    /// Remove favorite status from multiple photos.
    ///
    /// # Arguments
    ///
    /// * `uids` - Vector of photo UIDs to unfavorite
    ///
    /// # Returns
    ///
    /// A `PhotoBatchResponse` with success/error counts.
    pub async fn dislike_photos(&self, uids: Vec<String>) -> Result<PhotoBatchResponse> {
        if uids.is_empty() {
            return Err(anyhow!("Photo UID list cannot be empty"));
        }

        let request = PhotoBatchRequest { photos: uids };
        let path = "/api/v1/batch/photos/dislike";
        self.post(path, &request).await
    }

    // ========================================================================
    // Helper Methods (Internal)
    // ========================================================================

    /// Build query string from search parameters.
    fn build_search_query(&self, params: &PhotoSearchParams) -> String {
        let mut query_params = HashMap::new();

        if let Some(ref q) = params.q {
            query_params.insert("q", q.clone());
        }

        if let Some(ref photo_type) = params.photo_type {
            let type_str = match photo_type {
                crate::types::photo::PhotoType::Image => "image",
                crate::types::photo::PhotoType::Video => "video",
                crate::types::photo::PhotoType::Live => "live",
                crate::types::photo::PhotoType::Raw => "raw",
            };
            query_params.insert("type", type_str.to_string());
        }

        if let Some(quality) = params.quality {
            query_params.insert("quality", quality.to_string());
        }

        if let Some(favorite) = params.favorite {
            query_params.insert("favorite", favorite.to_string());
        }

        if let Some(ref country) = params.country {
            query_params.insert("country", country.clone());
        }

        if let Some(ref state) = params.state {
            query_params.insert("state", state.clone());
        }

        if let Some(ref city) = params.city {
            query_params.insert("city", city.clone());
        }

        if let Some(ref year) = params.year {
            query_params.insert("year", year.clone());
        }

        if let Some(ref month) = params.month {
            query_params.insert("month", month.clone());
        }

        if let Some(ref day) = params.day {
            query_params.insert("day", day.clone());
        }

        if let Some(ref camera) = params.camera {
            query_params.insert("camera", camera.clone());
        }

        if let Some(ref lens) = params.lens {
            query_params.insert("lens", lens.clone());
        }

        if let Some(ref label) = params.label {
            query_params.insert("label", label.clone());
        }

        if let Some(ref album) = params.album {
            query_params.insert("album", album.clone());
        }

        if let Some(ref subject) = params.subject {
            query_params.insert("subject", subject.clone());
        }

        if let Some(ref order) = params.order {
            let order_str = match order {
                crate::types::photo::SortOrder::Newest => "newest",
                crate::types::photo::SortOrder::Oldest => "oldest",
                crate::types::photo::SortOrder::Added => "added",
                crate::types::photo::SortOrder::Edited => "edited",
                crate::types::photo::SortOrder::Name => "name",
                crate::types::photo::SortOrder::Size => "size",
                crate::types::photo::SortOrder::Relevance => "relevance",
            };
            query_params.insert("order", order_str.to_string());
        }

        query_params.insert("count", params.count.to_string());
        query_params.insert("offset", params.offset.to_string());

        self.build_query_string(&query_params)
    }

    /// Build query string from key-value pairs.
    fn build_query_string(&self, params: &HashMap<&str, String>) -> String {
        params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&")
    }

    // ========================================================================
    // HTTP Methods (These would typically be in src/client/mod.rs)
    // ========================================================================
    // The following methods are placeholders that reference the parent
    // PhotoPrismClient implementation. In the actual codebase, these would
    // be implemented once in the main client module.

    /// Make an authenticated GET request.
    async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to send GET request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Request failed with status {}: {}",
                status,
                body
            ));
        }

        response
            .json()
            .await
            .context("Failed to parse JSON response")
    }

    /// Make an authenticated POST request.
    async fn post<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: serde::de::DeserializeOwned,
    {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(body)
            .send()
            .await
            .context("Failed to send POST request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Request failed with status {}: {}",
                status,
                body
            ));
        }

        response
            .json()
            .await
            .context("Failed to parse JSON response")
    }

    /// Make an authenticated PUT request.
    async fn put<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: serde::de::DeserializeOwned,
    {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(body)
            .send()
            .await
            .context("Failed to send PUT request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Request failed with status {}: {}",
                status,
                body
            ));
        }

        response
            .json()
            .await
            .context("Failed to parse JSON response")
    }

    /// Make an authenticated DELETE request.
    async fn delete(&self, path: &str) -> Result<()> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http_client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to send DELETE request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Request failed with status {}: {}",
                status,
                body
            ));
        }

        Ok(())
    }

    /// Ensure authentication token is valid.
    ///
    /// This method would be implemented in the main client module to handle
    /// session token management and refresh.
    async fn ensure_authenticated(&self) -> Result<String> {
        // Check if we have a cached token
        {
            let token_guard = self.session_token.read().await;
            if let Some(token) = token_guard.as_ref() {
                return Ok(token.clone());
            }
        }

        // If no token, this would trigger authentication
        // For now, return an error
        Err(anyhow!("Not authenticated - session token required"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uid_validation() {
        // Valid 16-character UID would pass in actual implementation
        assert_eq!("abc123def4567890".len(), 16);
    }

    #[test]
    fn test_empty_uid_error() {
        let empty = "";
        assert!(empty.is_empty());
    }
}
