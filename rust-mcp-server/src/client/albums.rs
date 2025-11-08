use super::PhotoPrismClient;
use crate::types::{
    Album, AlbumCreateParams, AlbumList, AlbumSearchParams, AlbumUpdateParams,
    AddPhotosParams, RemovePhotosParams,
};
use anyhow::Result;
use serde::Serialize;

impl PhotoPrismClient {
    /// List all albums with optional filters
    pub async fn list_albums(&self, params: Option<AlbumSearchParams>) -> Result<Vec<Album>> {
        let path = "/api/v1/albums";

        let query_params = params.unwrap_or_default();
        let mut url = format!("{}?count={}&offset={}", path, query_params.count, query_params.offset);

        if let Some(q) = &query_params.q {
            url.push_str(&format!("&q={}", urlencoding::encode(q)));
        }

        if let Some(album_type) = &query_params.album_type {
            let type_str = match album_type {
                crate::types::AlbumType::Album => "album",
                crate::types::AlbumType::Folder => "folder",
                crate::types::AlbumType::Moment => "moment",
                crate::types::AlbumType::Month => "month",
                crate::types::AlbumType::State => "state",
            };
            url.push_str(&format!("&type={}", type_str));
        }

        if let Some(category) = &query_params.category {
            url.push_str(&format!("&category={}", urlencoding::encode(category)));
        }

        if let Some(location) = &query_params.location {
            url.push_str(&format!("&location={}", urlencoding::encode(location)));
        }

        if let Some(country) = &query_params.country {
            url.push_str(&format!("&country={}", country));
        }

        if let Some(favorite) = query_params.favorite {
            url.push_str(&format!("&favorite={}", favorite));
        }

        if let Some(order) = &query_params.order {
            url.push_str(&format!("&order={}", order));
        }

        self.get(&url).await
    }

    /// Get a specific album by UID
    pub async fn get_album(&self, uid: &str) -> Result<Album> {
        let path = format!("/api/v1/albums/{}", uid);
        self.get(&path).await
    }

    /// Create a new album
    pub async fn create_album(&self, params: AlbumCreateParams) -> Result<Album> {
        let path = "/api/v1/albums";
        self.post(path, &params).await
    }

    /// Update an existing album
    pub async fn update_album(&self, uid: &str, params: AlbumUpdateParams) -> Result<Album> {
        let path = format!("/api/v1/albums/{}", uid);
        self.put(&path, &params).await
    }

    /// Delete an album
    pub async fn delete_album(&self, uid: &str) -> Result<()> {
        let path = format!("/api/v1/albums/{}", uid);
        self.delete(&path).await
    }

    /// Add photos to an album
    pub async fn add_photos_to_album(&self, album_uid: &str, photo_uids: Vec<String>) -> Result<Album> {
        let path = format!("/api/v1/albums/{}/photos", album_uid);

        #[derive(Serialize)]
        struct PhotoSelection {
            photos: Vec<String>,
        }

        let body = PhotoSelection {
            photos: photo_uids,
        };

        self.post(&path, &body).await
    }

    /// Remove photos from an album
    pub async fn remove_photos_from_album(&self, album_uid: &str, photo_uids: Vec<String>) -> Result<()> {
        let path = format!("/api/v1/albums/{}/photos", album_uid);

        #[derive(Serialize)]
        struct PhotoSelection {
            photos: Vec<String>,
        }

        let body = PhotoSelection {
            photos: photo_uids,
        };

        self.delete_with_body(&path, &body).await
    }

    /// Like/favorite an album
    pub async fn like_album(&self, uid: &str) -> Result<Album> {
        let path = format!("/api/v1/albums/{}/like", uid);
        self.post(&path, &()).await
    }

    /// Unlike/unfavorite an album
    pub async fn unlike_album(&self, uid: &str) -> Result<Album> {
        let path = format!("/api/v1/albums/{}/like", uid);
        self.delete(&path).await
    }

    /// Clone an album (create a copy)
    pub async fn clone_album(&self, uid: &str) -> Result<Album> {
        let path = format!("/api/v1/albums/{}/clone", uid);
        self.post(&path, &()).await
    }

    /// Download album as ZIP
    pub async fn download_album(&self, uid: &str) -> Result<Vec<u8>> {
        let path = format!("/api/v1/albums/{}/dl", uid);
        self.download(&path).await
    }
}

// Add urlencoding dependency
// Note: This would need to be added to Cargo.toml:
// urlencoding = "2.1"
