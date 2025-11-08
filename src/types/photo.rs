//! Photo data types for PhotoPrism MCP server.
//!
//! This module defines the core photo types used for API communication and tool parameters.
//! All types are designed with JSON schema generation in mind for optimal LLM interaction.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single photo in PhotoPrism.
///
/// This is the primary photo entity returned by the PhotoPrism API.
/// It contains comprehensive metadata including location, camera settings,
/// categorization, and user-defined attributes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct Photo {
    /// Unique identifier (16-character alphanumeric)
    #[serde(rename = "UID")]
    pub uid: String,

    /// Document ID (internal)
    #[serde(rename = "DocumentID")]
    pub document_id: Option<String>,

    /// Photo type (image, video, live, raw, etc.)
    #[serde(rename = "Type")]
    pub photo_type: String,

    /// Title of the photo
    #[serde(default)]
    pub title: String,

    /// Description or caption
    #[serde(default)]
    pub description: String,

    /// Original filename
    #[serde(rename = "OriginalName")]
    pub original_name: String,

    /// Favorite status
    #[serde(default)]
    pub favorite: bool,

    /// Private/hidden status
    #[serde(default)]
    pub private: bool,

    /// Photo quality score (1-7, where 7 is highest)
    #[schemars(range(min = 1, max = 7))]
    pub quality: i32,

    /// ISO sensitivity value
    #[serde(rename = "ISO")]
    pub iso: Option<i32>,

    /// Focal length in mm
    #[serde(rename = "FocalLength")]
    pub focal_length: Option<f64>,

    /// F-number (aperture)
    #[serde(rename = "FNumber")]
    pub f_number: Option<f64>,

    /// Exposure time (shutter speed) in seconds
    #[serde(rename = "Exposure")]
    pub exposure: Option<String>,

    /// Camera make (e.g., "Canon", "Nikon")
    #[serde(rename = "CameraMake")]
    pub camera_make: Option<String>,

    /// Camera model (e.g., "EOS 5D Mark IV")
    #[serde(rename = "CameraModel")]
    pub camera_model: Option<String>,

    /// Lens make
    #[serde(rename = "LensMake")]
    pub lens_make: Option<String>,

    /// Lens model
    #[serde(rename = "LensModel")]
    pub lens_model: Option<String>,

    /// Country name
    #[serde(rename = "Country")]
    pub country: Option<String>,

    /// Location latitude
    #[serde(rename = "Lat")]
    pub lat: Option<f64>,

    /// Location longitude
    #[serde(rename = "Lng")]
    pub lng: Option<f64>,

    /// Altitude in meters
    #[serde(rename = "Altitude")]
    pub altitude: Option<i32>,

    /// Location place name
    #[serde(rename = "Place")]
    pub place: Option<String>,

    /// Location state/province
    #[serde(rename = "State")]
    pub state: Option<String>,

    /// Location city
    #[serde(rename = "City")]
    pub city: Option<String>,

    /// Timestamp when photo was taken (ISO 8601)
    #[serde(rename = "TakenAt")]
    pub taken_at: Option<String>,

    /// Timestamp when photo was taken (local time zone)
    #[serde(rename = "TakenAtLocal")]
    pub taken_at_local: Option<String>,

    /// Local time zone
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,

    /// Photo width in pixels
    #[serde(rename = "Width")]
    pub width: Option<i32>,

    /// Photo height in pixels
    #[serde(rename = "Height")]
    pub height: Option<i32>,

    /// File hash (checksum)
    #[serde(rename = "Hash")]
    pub hash: Option<String>,

    /// File size in bytes
    #[serde(rename = "Bytes")]
    pub bytes: Option<i64>,

    /// Keywords/tags associated with the photo
    #[serde(default)]
    #[serde(rename = "Keywords")]
    pub keywords: Vec<String>,

    /// Labels/categories detected in the photo
    #[serde(default)]
    #[serde(rename = "Labels")]
    pub labels: Vec<PhotoLabel>,

    /// Files associated with this photo (originals, thumbnails, etc.)
    #[serde(default)]
    #[serde(rename = "Files")]
    pub files: Vec<PhotoFile>,
}

/// Label/category detected in a photo.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct PhotoLabel {
    /// Label unique identifier
    #[serde(rename = "UID")]
    pub uid: String,

    /// Label name (e.g., "sunset", "mountain", "portrait")
    pub name: String,

    /// Confidence score (0-100)
    #[schemars(range(min = 0, max = 100))]
    pub uncertainty: i32,

    /// Priority/importance (higher is more important)
    pub priority: i32,
}

/// File associated with a photo (original, sidecar, thumbnail, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct PhotoFile {
    /// File unique identifier
    #[serde(rename = "UID")]
    pub uid: String,

    /// File name
    pub name: String,

    /// File hash
    pub hash: String,

    /// File size in bytes
    pub size: i64,

    /// MIME type (e.g., "image/jpeg")
    pub mime: String,

    /// Whether this is the primary file
    pub primary: bool,

    /// File width in pixels
    pub width: Option<i32>,

    /// File height in pixels
    pub height: Option<i32>,

    /// File orientation (1-8, EXIF standard)
    pub orientation: Option<i32>,
}

/// Response containing a list of photos.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PhotoList {
    /// Array of photos
    #[serde(default)]
    pub photos: Vec<Photo>,

    /// Total count of matching photos (may be larger than returned array)
    pub count: Option<i32>,

    /// Offset used for pagination
    pub offset: Option<i32>,
}

/// Parameters for advanced photo search.
///
/// This struct supports all major PhotoPrism search filters and sorting options.
/// Use this for complex queries that require multiple criteria.
///
/// # Examples
///
/// Search for favorite photos:
/// ```json
/// {
///   "favorite": true,
///   "count": 50
/// }
/// ```
///
/// Search by quality and type:
/// ```json
/// {
///   "quality": 5,
///   "type": "image",
///   "count": 100
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PhotoSearchParams {
    /// Text search query (searches titles, descriptions, keywords)
    ///
    /// Supports PhotoPrism query syntax including:
    /// - Exact phrases: "golden gate bridge"
    /// - Negation: -portrait
    /// - Wildcards: sun*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,

    /// Filter by photo type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub photo_type: Option<PhotoType>,

    /// Filter by quality (1-7, where 7 is best)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1, max = 7))]
    pub quality: Option<u8>,

    /// Only return favorite photos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,

    /// Filter by country name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Filter by state/province
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Filter by city
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Filter by year taken (YYYY format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,

    /// Filter by month taken (YYYY-MM format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,

    /// Filter by specific day (YYYY-MM-DD format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,

    /// Filter by camera make
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<String>,

    /// Filter by lens model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lens: Option<String>,

    /// Filter by label/category (e.g., "sunset", "portrait")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Filter by album UID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    /// Filter by subject/person UID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SortOrder>,

    /// Maximum number of results to return (default: 100, max: 1000)
    #[serde(default = "default_count")]
    #[schemars(range(min = 1, max = 1000))]
    pub count: u32,

    /// Pagination offset (default: 0)
    #[serde(default)]
    pub offset: u32,
}

fn default_count() -> u32 {
    100
}

/// Photo type filter options.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PhotoType {
    /// Still images (JPEG, PNG, etc.)
    Image,
    /// Video files
    Video,
    /// Live photos (iOS format)
    Live,
    /// RAW camera files
    Raw,
}

/// Sort order options for photo queries.
///
/// Controls how results are ordered in the response.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Newest first (by taken date)
    Newest,
    /// Oldest first (by taken date)
    Oldest,
    /// Recently added to library
    Added,
    /// Recently edited/updated
    Edited,
    /// By filename
    Name,
    /// By file size
    Size,
    /// By relevance (for search queries)
    Relevance,
}

/// Parameters for updating photo metadata.
///
/// All fields are optional - only provided fields will be updated.
/// Use this to modify photo titles, descriptions, favorites, and other attributes.
///
/// # Examples
///
/// Mark as favorite:
/// ```json
/// {
///   "favorite": true
/// }
/// ```
///
/// Update title and description:
/// ```json
/// {
///   "title": "Golden Gate Bridge at Sunset",
///   "description": "Beautiful sunset view from Battery Spencer"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct PhotoUpdate {
    /// New title for the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// New description/caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set favorite status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,

    /// Set private/hidden status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,

    /// Override photo quality (1-7)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1, max = 7))]
    pub quality: Option<u8>,

    /// Update keywords/tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    /// Update latitude
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Lat")]
    pub lat: Option<f64>,

    /// Update longitude
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Lng")]
    pub lng: Option<f64>,

    /// Update altitude in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<i32>,

    /// Update country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Update place name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
}

/// Batch operation request for multiple photos.
///
/// Used to perform the same operation on multiple photos at once.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PhotoBatchRequest {
    /// Array of photo UIDs to operate on
    #[schemars(length(min = 1))]
    pub photos: Vec<String>,
}

/// Response from a batch operation.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PhotoBatchResponse {
    /// Number of photos successfully processed
    pub count: i32,

    /// Number of photos that failed
    pub errors: i32,

    /// Detailed error messages (if any)
    #[serde(default)]
    pub messages: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use schemars::schema_for;

    #[test]
    fn test_photo_search_params_schema() {
        let schema = schema_for!(PhotoSearchParams);
        assert!(schema.schema.object.is_some());
    }

    #[test]
    fn test_photo_update_schema() {
        let schema = schema_for!(PhotoUpdate);
        assert!(schema.schema.object.is_some());
    }

    #[test]
    fn test_default_count() {
        let params = PhotoSearchParams {
            q: None,
            photo_type: None,
            quality: None,
            favorite: None,
            country: None,
            state: None,
            city: None,
            year: None,
            month: None,
            day: None,
            camera: None,
            lens: None,
            label: None,
            album: None,
            subject: None,
            order: None,
            count: default_count(),
            offset: 0,
        };
        assert_eq!(params.count, 100);
    }
}
