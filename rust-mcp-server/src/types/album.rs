use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Album type distinguishes different kinds of albums
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AlbumType {
    /// Manually created album
    Album,
    /// Auto-generated folder album
    Folder,
    /// Moment (time-based collection)
    Moment,
    /// Month album
    Month,
    /// State/location-based album
    State,
}

impl Default for AlbumType {
    fn default() -> Self {
        AlbumType::Album
    }
}

/// Album represents a photo collection in PhotoPrism
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct Album {
    /// Unique identifier
    #[serde(rename = "UID")]
    pub uid: String,

    /// Parent album UID (for nested albums)
    #[serde(rename = "ParentUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uid: Option<String>,

    /// Album title
    pub title: String,

    /// URL-friendly slug
    pub slug: String,

    /// Album type
    #[serde(rename = "Type")]
    pub album_type: AlbumType,

    /// Album description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Location name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Filter criteria for smart albums
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Country code (ISO 3166-1 alpha-2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// State/province
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Year
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,

    /// Month (1-12)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1, max = 12))]
    pub month: Option<i32>,

    /// Day of month (1-31)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1, max = 31))]
    pub day: Option<i32>,

    /// Is this a favorite album?
    #[serde(default)]
    pub favorite: bool,

    /// Is this album private?
    #[serde(default)]
    pub private: bool,

    /// Number of photos in album
    #[serde(rename = "PhotoCount")]
    #[serde(default)]
    pub photo_count: u32,

    /// Number of links
    #[serde(rename = "LinkCount")]
    #[serde(default)]
    pub link_count: u32,

    /// Thumbnail hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,

    /// Thumbnail source
    #[serde(rename = "ThumbSrc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_src: Option<String>,

    /// File path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Creation timestamp
    #[serde(rename = "CreatedAt")]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    #[serde(rename = "UpdatedAt")]
    pub updated_at: DateTime<Utc>,
}

/// Response containing a list of albums
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlbumList {
    /// List of albums
    pub albums: Vec<Album>,

    /// Total count (for pagination)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Parameters for creating a new album
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AlbumCreateParams {
    /// Album title (required)
    #[schemars(length(min = 1, max = 255))]
    pub title: String,

    /// Album description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 2000))]
    pub description: Option<String>,

    /// Category
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 100))]
    pub category: Option<String>,

    /// Location
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 255))]
    pub location: Option<String>,

    /// Make this album favorite
    #[serde(default)]
    pub favorite: bool,

    /// Make this album private
    #[serde(default)]
    pub private: bool,
}

/// Parameters for updating an existing album
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct AlbumUpdateParams {
    /// New title
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(min = 1, max = 255))]
    pub title: Option<String>,

    /// New description
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 2000))]
    pub description: Option<String>,

    /// New category
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 100))]
    pub category: Option<String>,

    /// New location
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(max = 255))]
    pub location: Option<String>,

    /// Update favorite status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,

    /// Update private status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,

    /// Update sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// Parameters for adding photos to an album
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPhotosParams {
    /// Album UID to add photos to
    #[schemars(length(min = 16, max = 16))]
    pub album_uid: String,

    /// List of photo UIDs to add (max 100 per request)
    #[schemars(length(min = 1, max = 100))]
    pub photo_uids: Vec<String>,
}

/// Parameters for removing photos from an album
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemovePhotosParams {
    /// Album UID to remove photos from
    #[schemars(length(min = 16, max = 16))]
    pub album_uid: String,

    /// List of photo UIDs to remove (max 100 per request)
    #[schemars(length(min = 1, max = 100))]
    pub photo_uids: Vec<String>,
}

/// Search/filter parameters for listing albums
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AlbumSearchParams {
    /// Search query for album title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,

    /// Filter by album type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_type: Option<AlbumType>,

    /// Filter by category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Filter by location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Filter by country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Show only favorites
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,

    /// Maximum number of results (default: 100, max: 1000)
    #[serde(default = "default_count")]
    #[schemars(range(min = 1, max = 1000))]
    pub count: u32,

    /// Offset for pagination
    #[serde(default)]
    pub offset: u32,

    /// Sort order (newest, oldest, name, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

fn default_count() -> u32 {
    100
}

impl Default for AlbumSearchParams {
    fn default() -> Self {
        Self {
            q: None,
            album_type: None,
            category: None,
            location: None,
            country: None,
            favorite: None,
            count: default_count(),
            offset: 0,
            order: None,
        }
    }
}
