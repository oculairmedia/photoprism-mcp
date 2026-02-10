use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::{DateTime, Utc};

/// Photo type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PhotoType {
    Image,
    Video,
    Live,
    Raw,
}

/// Photo information from PhotoPrism API
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct Photo {
    /// Photo UID
    #[serde(rename = "UID")]
    pub uid: String,

    /// Photo title
    #[serde(default)]
    pub title: String,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Original filename
    #[serde(default)]
    pub original_name: String,

    /// Photo type (image, video, live, raw)
    #[serde(rename = "Type")]
    pub photo_type: String,

    /// Favorite status
    #[serde(default)]
    pub favorite: bool,

    /// Private status
    #[serde(default)]
    pub private: bool,

    /// Taken at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taken_at: Option<DateTime<Utc>>,

    /// Camera make
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_make: Option<String>,

    /// Camera model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_model: Option<String>,

    /// Latitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,

    /// Longitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,

    /// File hash
    #[serde(default)]
    pub hash: String,

    /// Width in pixels
    #[serde(default)]
    pub width: u32,

    /// Height in pixels
    #[serde(default)]
    pub height: u32,
}

/// Photo search filters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PhotoSearchParams {
    /// Search query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,

    /// Quality filter (1-7, where 3+ is good quality)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1, max = 7))]
    pub quality: Option<u8>,

    /// Filter by album UID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    /// Filter by label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Filter by year
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,

    /// Filter by month (1-12)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1, max = 12))]
    pub month: Option<u8>,

    /// Filter by photo type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_type: Option<PhotoType>,

    /// Maximum results (default: 100, max: 1000)
    #[serde(default = "default_count")]
    #[schemars(range(min = 1, max = 1000))]
    pub count: u32,

    /// Offset for pagination
    #[serde(default)]
    pub offset: u32,
}

fn default_count() -> u32 {
    100
}

/// Photo update request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct PhotoUpdate {
    /// New title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Favorite status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,

    /// Private status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

/// Photo list response
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PhotoList {
    /// List of photos
    pub photos: Vec<Photo>,

    /// Total count
    pub count: u32,
}
