use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Status of a single item in a batch operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BatchItemStatus {
    /// Operation succeeded for this item
    Success,
    /// Operation failed for this item
    Error,
    /// Operation was skipped (e.g., validation failed)
    Skipped,
}

/// Result of a single item in a batch operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchItemResult {
    /// Item identifier (photo UID, album UID, etc.)
    pub id: String,

    /// Item name or title (for better readability)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Status of this item's operation
    pub status: BatchItemStatus,

    /// Error message if status is Error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Additional details about the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl BatchItemResult {
    /// Create a success result
    pub fn success(id: String, name: Option<String>) -> Self {
        Self {
            id,
            name,
            status: BatchItemStatus::Success,
            error: None,
            details: None,
        }
    }

    /// Create an error result
    pub fn error(id: String, name: Option<String>, error: String) -> Self {
        Self {
            id,
            name,
            status: BatchItemStatus::Error,
            error: Some(error),
            details: None,
        }
    }

    /// Create a skipped result
    pub fn skipped(id: String, name: Option<String>, reason: String) -> Self {
        Self {
            id,
            name,
            status: BatchItemStatus::Skipped,
            error: None,
            details: Some(reason),
        }
    }
}

/// Summary of a batch operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchSummary {
    /// Total number of items processed
    pub total: usize,

    /// Number of successful operations
    pub success_count: usize,

    /// Number of failed operations
    pub error_count: usize,

    /// Number of skipped items
    pub skipped_count: usize,

    /// Overall success rate (0.0 to 1.0)
    pub success_rate: f64,
}

impl BatchSummary {
    /// Create a summary from results
    pub fn from_results(results: &[BatchItemResult]) -> Self {
        let total = results.len();
        let success_count = results
            .iter()
            .filter(|r| r.status == BatchItemStatus::Success)
            .count();
        let error_count = results
            .iter()
            .filter(|r| r.status == BatchItemStatus::Error)
            .count();
        let skipped_count = results
            .iter()
            .filter(|r| r.status == BatchItemStatus::Skipped)
            .count();

        let success_rate = if total > 0 {
            success_count as f64 / total as f64
        } else {
            0.0
        };

        Self {
            total,
            success_count,
            error_count,
            skipped_count,
            success_rate,
        }
    }
}

/// Complete response for a batch operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchOperationResponse {
    /// Summary of the operation
    pub summary: BatchSummary,

    /// Detailed results for each item
    pub results: Vec<BatchItemResult>,

    /// Optional message describing the overall operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl BatchOperationResponse {
    /// Create a response from results
    pub fn from_results(results: Vec<BatchItemResult>, message: Option<String>) -> Self {
        let summary = BatchSummary::from_results(&results);
        Self {
            summary,
            results,
            message,
        }
    }
}

/// Parameters for batch archive operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchArchiveParams {
    /// List of photo UIDs to archive (max 100 per request)
    #[schemars(length(min = 1, max = 100))]
    pub photo_uids: Vec<String>,

    /// Whether to restore (unarchive) instead of archive
    #[serde(default)]
    pub restore: bool,
}

/// Parameters for batch delete operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchDeleteParams {
    /// List of photo UIDs to delete (max 50 per request for safety)
    #[schemars(length(min = 1, max = 50))]
    pub photo_uids: Vec<String>,

    /// Require confirmation to prevent accidental deletion
    /// Must be set to true to proceed
    #[serde(default)]
    pub confirm: bool,

    /// Whether to permanently delete (vs move to trash)
    #[serde(default)]
    pub permanent: bool,
}

/// Parameters for batch favorite/unfavorite operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchFavoriteParams {
    /// List of photo UIDs to favorite/unfavorite (max 100 per request)
    #[schemars(length(min = 1, max = 100))]
    pub photo_uids: Vec<String>,

    /// Whether to unfavorite instead of favorite
    #[serde(default)]
    pub unfavorite: bool,
}

/// Parameters for batch update operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchUpdateParams {
    /// List of photo UIDs to update (max 50 per request)
    #[schemars(length(min = 1, max = 50))]
    pub photo_uids: Vec<String>,

    /// New title to apply (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// New description to apply (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Tags to add (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<String>>,

    /// Tags to remove (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
}

/// Parameters for batch private/public operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchPrivateParams {
    /// List of photo UIDs to make private/public (max 100 per request)
    #[schemars(length(min = 1, max = 100))]
    pub photo_uids: Vec<String>,

    /// Whether to make public instead of private
    #[serde(default)]
    pub make_public: bool,
}

/// Operation type for batch operations
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BatchOperationType {
    Archive,
    Delete,
    Favorite,
    Private,
    Update,
    AddToAlbum,
    RemoveFromAlbum,
}

/// Generic batch operation parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BatchOperationParams {
    /// Type of operation to perform
    pub operation: BatchOperationType,

    /// List of photo UIDs to operate on
    pub photo_uids: Vec<String>,

    /// Additional parameters as JSON (operation-specific)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}
