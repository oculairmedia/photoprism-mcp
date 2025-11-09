use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Album type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AlbumType {
    Album,
    Folder,
    Moment,
    Month,
    State,
}

/// Album information from PhotoPrism API
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct Album {
    /// Album UID
    #[serde(rename = "UID")]
    pub uid: String,

    /// Album title
    pub title: String,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Album type
    #[serde(rename = "Type")]
    pub album_type: String,

    /// Favorite status
    #[serde(default)]
    pub favorite: bool,

    /// Number of photos in album
    #[serde(default)]
    pub photo_count: u32,

    /// Created at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// Updated at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Album creation request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AlbumCreate {
    /// Album title
    pub title: String,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Favorite status
    #[serde(default)]
    pub favorite: bool,
}

/// Album update request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AlbumUpdate {
    /// New title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Favorite status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
}

/// Album list response
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlbumList {
    /// List of albums
    pub albums: Vec<Album>,

    /// Total count
    pub count: u32,
}

/// Request to add photos to album
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPhotosRequest {
    /// List of photo UIDs to add
    pub photos: Vec<String>,
}
