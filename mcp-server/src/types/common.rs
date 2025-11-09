use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Common search parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Search query string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,

    /// Maximum number of results (default: 100, max: 1000)
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

/// Label information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Label {
    /// Label UID
    pub uid: String,

    /// Label name
    pub name: String,

    /// Number of photos with this label
    #[serde(default)]
    pub photo_count: u32,

    /// Priority (0-100)
    #[serde(default)]
    pub priority: u8,
}

/// Subject/Person information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Subject {
    /// Subject UID
    pub uid: String,

    /// Subject name
    pub name: String,

    /// Number of photos with this subject
    #[serde(default)]
    pub photo_count: u32,

    /// Favorite status
    #[serde(default)]
    pub favorite: bool,
}

/// System status information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SystemStatus {
    /// PhotoPrism version
    pub version: String,

    /// System edition
    pub edition: String,

    /// Total number of photos
    pub photos: u64,

    /// Total number of albums
    pub albums: u64,

    /// Total number of labels
    pub labels: u64,
}

/// Sort order options
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum SortOrder {
    #[default]
    Newest,
    Oldest,
    Name,
    Size,
    Duration,
}

