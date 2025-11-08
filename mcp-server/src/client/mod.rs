use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::error::{PhotoPrismError, Result};
use crate::types::*;

/// PhotoPrism API client
pub struct PhotoPrismClient {
    http_client: Client,
    base_url: String,
    username: String,
    password: String,
    session_token: Arc<RwLock<Option<String>>>,
}

impl PhotoPrismClient {
    /// Create a new PhotoPrism API client
    pub fn new(
        base_url: String,
        username: String,
        password: String,
    ) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| PhotoPrismError::NetworkError(e))?;

        Ok(Self {
            http_client,
            base_url: base_url.trim_end_matches('/').to_string(),
            username,
            password,
            session_token: Arc::new(RwLock::new(None)),
        })
    }

    /// Ensure we have a valid session token
    async fn ensure_authenticated(&self) -> Result<String> {
        // Check if we have a cached token
        {
            let token_guard = self.session_token.read().await;
            if let Some(token) = token_guard.as_ref() {
                return Ok(token.clone());
            }
        }

        // Login to get a new token
        #[derive(Serialize)]
        struct LoginRequest {
            username: String,
            password: String,
        }

        #[derive(Deserialize)]
        struct LoginResponse {
            id: String,
        }

        let login_url = format!("{}/api/v1/session", self.base_url);
        let response = self.http_client
            .post(&login_url)
            .json(&LoginRequest {
                username: self.username.clone(),
                password: self.password.clone(),
            })
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PhotoPrismError::AuthenticationFailed(
                format!("Authentication failed: {}", response.status())
            ));
        }

        let login_response: LoginResponse = response.json().await?;
        let token = login_response.id;

        // Cache the token
        {
            let mut token_guard = self.session_token.write().await;
            *token_guard = Some(token.clone());
        }

        Ok(token)
    }

    /// Make an authenticated GET request
    async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
    ) -> Result<T> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self.http_client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            if status == 404 {
                return Err(PhotoPrismError::NotFound(
                    format!("Resource not found: {}", path)
                ));
            }
            return Err(PhotoPrismError::ApiError(
                format!("Request failed: {}", status)
            ));
        }

        Ok(response.json().await?)
    }

    /// Make an authenticated POST request
    pub async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self.http_client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PhotoPrismError::ApiError(
                format!("Request failed: {}", response.status())
            ));
        }

        Ok(response.json().await?)
    }

    /// Make an authenticated PUT request
    pub async fn put<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self.http_client
            .put(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PhotoPrismError::ApiError(
                format!("Request failed: {}", response.status())
            ));
        }

        Ok(response.json().await?)
    }

    /// Make an authenticated DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self.http_client
            .delete(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PhotoPrismError::ApiError(
                format!("Request failed: {}", response.status())
            ));
        }

        Ok(())
    }

    // Photo operations

    /// Search photos
    pub async fn search_photos(
        &self,
        query: &str,
        count: u32,
    ) -> Result<Vec<Photo>> {
        let path = format!("/api/v1/photos?q={}&count={}", query, count);
        self.get(&path).await
    }

    /// Get photo by UID
    pub async fn get_photo(&self, uid: &str) -> Result<Photo> {
        let path = format!("/api/v1/photos/{}", uid);
        self.get(&path).await
    }

    /// Update photo metadata
    pub async fn update_photo(
        &self,
        uid: &str,
        updates: &PhotoUpdate,
    ) -> Result<Photo> {
        let path = format!("/api/v1/photos/{}", uid);
        self.put(&path, updates).await
    }

    /// Delete photo
    pub async fn delete_photo(&self, uid: &str) -> Result<()> {
        let path = format!("/api/v1/photos/{}", uid);
        self.delete(&path).await
    }

    // Album operations

    /// List all albums
    pub async fn list_albums(&self) -> Result<Vec<Album>> {
        let path = "/api/v1/albums?count=1000";
        self.get(&path).await
    }

    /// Get album by UID
    pub async fn get_album(&self, uid: &str) -> Result<Album> {
        let path = format!("/api/v1/albums/{}", uid);
        self.get(&path).await
    }

    /// Create a new album
    pub async fn create_album(&self, create: &AlbumCreate) -> Result<Album> {
        let path = "/api/v1/albums";
        self.post(&path, create).await
    }

    /// Update album
    pub async fn update_album(
        &self,
        uid: &str,
        updates: &AlbumUpdate,
    ) -> Result<Album> {
        let path = format!("/api/v1/albums/{}", uid);
        self.put(&path, updates).await
    }

    /// Delete album
    pub async fn delete_album(&self, uid: &str) -> Result<()> {
        let path = format!("/api/v1/albums/{}", uid);
        self.delete(&path).await
    }

    /// Add photos to album
    pub async fn add_photos_to_album(
        &self,
        album_uid: &str,
        photo_uids: &[String],
    ) -> Result<()> {
        let path = format!("/api/v1/albums/{}/photos", album_uid);

        #[derive(Serialize)]
        struct AddPhotosRequest {
            photos: Vec<String>,
        }

        let _: serde_json::Value = self.post(&path, &AddPhotosRequest {
            photos: photo_uids.to_vec(),
        }).await?;

        Ok(())
    }

    // Label operations

    /// List all labels
    pub async fn list_labels(&self) -> Result<Vec<Label>> {
        let path = "/api/v1/labels?count=1000";
        self.get(&path).await
    }

    /// Get photos by label
    pub async fn get_photos_by_label(&self, label: &str, count: u32) -> Result<Vec<Photo>> {
        let path = format!("/api/v1/photos?label={}&count={}", label, count);
        self.get(&path).await
    }

    // Subject operations

    /// List all subjects (people)
    pub async fn list_subjects(&self) -> Result<Vec<Subject>> {
        let path = "/api/v1/subjects?count=1000";
        self.get(&path).await
    }

    /// Get photos by subject
    pub async fn get_photos_by_subject(&self, subject_uid: &str, count: u32) -> Result<Vec<Photo>> {
        let path = format!("/api/v1/photos?subject={}&count={}", subject_uid, count);
        self.get(&path).await
    }

    // System operations

    /// Get system status
    pub async fn get_status(&self) -> Result<SystemStatus> {
        let path = "/api/v1/status";

        #[derive(Deserialize)]
        struct StatusResponse {
            version: String,
            edition: String,
        }

        let status: StatusResponse = self.get(&path).await?;

        // Get counts from different endpoints
        let photos: Vec<Photo> = self.get("/api/v1/photos?count=0").await.unwrap_or_default();
        let albums: Vec<Album> = self.get("/api/v1/albums?count=0").await.unwrap_or_default();
        let labels: Vec<Label> = self.get("/api/v1/labels?count=0").await.unwrap_or_default();

        Ok(SystemStatus {
            version: status.version,
            edition: status.edition,
            photos: photos.len() as u64,
            albums: albums.len() as u64,
            labels: labels.len() as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = PhotoPrismClient::new(
            "http://localhost:2342".to_string(),
            "admin".to_string(),
            "password".to_string(),
        );
        assert!(client.is_ok());
    }
}
