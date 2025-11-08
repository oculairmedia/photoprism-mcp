//! Photo management tools for PhotoPrism MCP server.
//!
//! This module provides MCP tool handlers for photo operations including search,
//! retrieval, updating, and deletion. All tools are designed with LLM usability
//! in mind, following MCP best practices for tool descriptions and parameter design.
//!
//! # Tool Design Philosophy
//!
//! - **Workflow-based**: Each tool represents a complete user intention
//! - **Clear descriptions**: LLM can easily understand when to use each tool
//! - **Context-efficient**: Return only relevant information
//! - **Error-friendly**: Provide actionable error messages
//!
//! # Example Usage in Conversation
//!
//! User: "Show me my favorite sunset photos from 2023"
//! Assistant uses: `search_photos_advanced` with favorite=true, q="sunset", year="2023"
//!
//! User: "Find the photo I took at the Golden Gate Bridge"
//! Assistant uses: `search_photos` with query="Golden Gate Bridge"
//!
//! User: "Mark photo abc123 as a favorite"
//! Assistant uses: `update_photo` with uid="abc123", favorite=true

use crate::client::PhotoPrismClient;
use crate::types::photo::{PhotoSearchParams, PhotoUpdate};
use turbomcp::prelude::*;

// Note: These tools would be implemented as methods on the PhotoPrismServer struct
// which is defined in src/server/photoprism.rs. The #[tool] macro automatically
// registers them with the MCP server.
//
// Below are the tool implementations following TurboMCP patterns.

/// Search photos using a simple text query.
///
/// Use this tool when the user wants to find photos by searching titles, descriptions,
/// file names, or keywords. This is the simplest search method - for advanced filtering
/// (by date, location, quality, type, etc.), use `search_photos_advanced` instead.
///
/// # When to Use
///
/// - User asks to "find photos of..."
/// - User searches for specific subject matter ("sunset", "beach", "family")
/// - User mentions searching by location name
/// - Simple, single-criteria searches
///
/// # Examples
///
/// User: "Find photos of mountains"
/// → search_photos(query="mountains", count=50)
///
/// User: "Show me beach photos"
/// → search_photos(query="beach", count=100)
///
/// User: "Search for vacation pictures"
/// → search_photos(query="vacation", count=100)
///
/// # Parameters
///
/// - `query`: Search text (can be empty to get all photos)
/// - `count`: Maximum results to return (default: 100, max: 1000)
///
/// # Returns
///
/// JSON array of photos with metadata including:
/// - UID (for further operations)
/// - Title and description
/// - Location information
/// - Camera/lens details
/// - Quality score
/// - Labels and keywords
#[allow(dead_code)]
pub async fn search_photos_tool(
    client: &PhotoPrismClient,
    ctx: Context,
    query: String,
    count: Option<u32>,
) -> McpResult<String> {
    let count = count.unwrap_or(100).min(1000);

    ctx.info(&format!(
        "Searching photos with query: '{}', count: {}",
        query, count
    ))
    .await?;

    let results = client
        .search_photos(&query, count)
        .await
        .map_err(|e| McpError::internal(format!("Photo search failed: {}", e)))?;

    ctx.info(&format!("Found {} photos", results.photos.len()))
        .await?;

    serde_json::to_string_pretty(&results)
        .map_err(|e| McpError::internal(format!("Failed to serialize results: {}", e)))
}

/// Search photos with advanced filters and criteria.
///
/// Use this tool when the user needs to search with multiple criteria or specific
/// filters beyond simple text search. Supports filtering by type, quality, favorites,
/// location, date ranges, camera equipment, and more.
///
/// # When to Use
///
/// - User specifies multiple search criteria
/// - User asks for photos by date/time ("photos from 2023", "pictures in June")
/// - User filters by location ("photos from California", "pictures in Paris")
/// - User wants favorites only ("show my favorite photos")
/// - User filters by quality ("best photos", "high quality images")
/// - User specifies photo type ("videos", "raw files", "live photos")
/// - User combines multiple filters ("favorite sunset photos from last summer")
///
/// # Examples
///
/// User: "Show my favorite photos from 2023"
/// → search_photos_advanced(favorite=true, year="2023", count=100)
///
/// User: "Find high quality sunset photos"
/// → search_photos_advanced(q="sunset", quality=5, count=50)
///
/// User: "Show me all videos from California"
/// → search_photos_advanced(type="video", state="California", count=100)
///
/// User: "Pictures I took with my Canon camera in December 2022"
/// → search_photos_advanced(camera="Canon", year="2022", month="2022-12", count=100)
///
/// # Parameters
///
/// All parameters are optional - only specify those mentioned by the user:
///
/// - `q`: Text search query
/// - `photo_type`: Filter by type (image, video, live, raw)
/// - `quality`: Quality score filter (1-7, where 7 is best)
/// - `favorite`: Only favorite photos (true/false)
/// - `country`: Filter by country name
/// - `state`: Filter by state/province
/// - `city`: Filter by city
/// - `year`: Filter by year (YYYY format, e.g., "2023")
/// - `month`: Filter by month (YYYY-MM format, e.g., "2023-06")
/// - `day`: Filter by specific day (YYYY-MM-DD format)
/// - `camera`: Filter by camera make/model
/// - `lens`: Filter by lens model
/// - `label`: Filter by detected label/category
/// - `album`: Filter by album UID
/// - `subject`: Filter by person/subject UID
/// - `order`: Sort order (newest, oldest, added, edited, name, size, relevance)
/// - `count`: Maximum results (default: 100, max: 1000)
/// - `offset`: Pagination offset (default: 0)
///
/// # Returns
///
/// JSON array of photos matching all specified criteria.
#[allow(dead_code)]
pub async fn search_photos_advanced_tool(
    client: &PhotoPrismClient,
    ctx: Context,
    params: PhotoSearchParams,
) -> McpResult<String> {
    ctx.info(&format!(
        "Advanced photo search - count: {}, offset: {}",
        params.count, params.offset
    ))
    .await?;

    let results = client
        .search_photos_advanced(params)
        .await
        .map_err(|e| McpError::internal(format!("Advanced search failed: {}", e)))?;

    ctx.info(&format!("Found {} photos", results.photos.len()))
        .await?;

    serde_json::to_string_pretty(&results)
        .map_err(|e| McpError::internal(format!("Failed to serialize results: {}", e)))
}

/// Get detailed information about a specific photo.
///
/// Use this tool when the user wants to see all details about a particular photo
/// they've already identified (usually from a previous search). Returns complete
/// metadata including EXIF data, location, labels, files, and more.
///
/// # When to Use
///
/// - User asks for "details" or "information" about a specific photo
/// - User references a photo UID from a previous search
/// - User wants to see EXIF data, camera settings, or technical details
/// - User asks "tell me more about photo xyz"
/// - Before updating or deleting a photo (to confirm it's the right one)
///
/// # Examples
///
/// User: "Tell me more about photo abc123def4567890"
/// → get_photo(uid="abc123def4567890")
///
/// User: "What are the details of that sunset photo?" (after getting UID from search)
/// → get_photo(uid="<uid_from_search>")
///
/// User: "Show me the EXIF data for this image"
/// → get_photo(uid="<uid>")
///
/// # Parameters
///
/// - `uid`: Photo unique identifier (16-character alphanumeric string)
///
/// # Returns
///
/// Complete photo metadata including:
/// - Title, description, keywords
/// - EXIF data (ISO, aperture, shutter speed, focal length)
/// - Camera and lens information
/// - GPS coordinates and location details
/// - Quality score and file information
/// - Detected labels and categories
/// - Associated files (originals, thumbnails, etc.)
///
/// # Errors
///
/// - Returns error if photo not found
/// - Returns error if UID format is invalid (must be 16 characters)
#[allow(dead_code)]
pub async fn get_photo_tool(
    client: &PhotoPrismClient,
    ctx: Context,
    uid: String,
) -> McpResult<String> {
    // Validate UID format
    if uid.is_empty() {
        return Err(McpError::invalid_request(
            "Photo UID cannot be empty. Please provide a 16-character photo identifier.",
        ));
    }

    if uid.len() != 16 {
        return Err(McpError::invalid_request(format!(
            "Invalid UID format: expected 16 characters, got {}. Photo UIDs should be 16-character alphanumeric strings.",
            uid.len()
        )));
    }

    ctx.info(&format!("Retrieving photo: {}", uid)).await?;

    let photo = client
        .get_photo(&uid)
        .await
        .map_err(|e| McpError::not_found(format!("Photo not found: {}", e)))?;

    ctx.info(&format!("Retrieved photo: '{}'", photo.title))
        .await?;

    serde_json::to_string_pretty(&photo)
        .map_err(|e| McpError::internal(format!("Failed to serialize photo: {}", e)))
}

/// Update photo metadata and attributes.
///
/// Use this tool when the user wants to modify a photo's title, description,
/// favorite status, location, or other editable attributes. Only the fields
/// provided will be updated - other fields remain unchanged.
///
/// # When to Use
///
/// - User wants to rename a photo ("change the title to...")
/// - User wants to add/edit description
/// - User wants to mark/unmark as favorite ("make this a favorite")
/// - User wants to update location information
/// - User wants to add keywords/tags
/// - User wants to change quality rating
/// - User says "edit", "update", "change", or "modify" photo
///
/// # Examples
///
/// User: "Mark photo abc123 as a favorite"
/// → update_photo(uid="abc123", favorite=true)
///
/// User: "Change the title of photo xyz789 to 'Beautiful Sunset'"
/// → update_photo(uid="xyz789", title="Beautiful Sunset")
///
/// User: "Add a description to that photo"
/// → update_photo(uid="<uid>", description="Description text here")
///
/// User: "Remove favorite from photo abc123"
/// → update_photo(uid="abc123", favorite=false)
///
/// User: "Update the location of photo xyz to San Francisco"
/// → update_photo(uid="xyz", city="San Francisco")
///
/// # Parameters
///
/// - `uid`: Photo unique identifier (16 characters, required)
/// - `title`: New title for the photo (optional)
/// - `description`: New description/caption (optional)
/// - `favorite`: Set favorite status true/false (optional)
/// - `private`: Set private/hidden status true/false (optional)
/// - `quality`: Override quality score 1-7 (optional)
/// - `keywords`: Array of keywords/tags (optional)
/// - `lat`: Latitude coordinate (optional)
/// - `lng`: Longitude coordinate (optional)
/// - `altitude`: Altitude in meters (optional)
/// - `country`: Country name (optional)
/// - `place`: Place name (optional)
///
/// All parameters except `uid` are optional. Only provide fields that should be updated.
///
/// # Returns
///
/// The updated photo object with all current metadata.
///
/// # Errors
///
/// - Returns error if photo not found
/// - Returns error if UID is invalid
/// - Returns error if update fails (e.g., invalid quality value)
#[allow(dead_code)]
pub async fn update_photo_tool(
    client: &PhotoPrismClient,
    ctx: Context,
    uid: String,
    title: Option<String>,
    description: Option<String>,
    favorite: Option<bool>,
    private: Option<bool>,
    quality: Option<u8>,
    keywords: Option<Vec<String>>,
    lat: Option<f64>,
    lng: Option<f64>,
    altitude: Option<i32>,
    country: Option<String>,
    place: Option<String>,
) -> McpResult<String> {
    // Validate UID format
    if uid.is_empty() {
        return Err(McpError::invalid_request(
            "Photo UID cannot be empty. Please provide a 16-character photo identifier.",
        ));
    }

    if uid.len() != 16 {
        return Err(McpError::invalid_request(format!(
            "Invalid UID format: expected 16 characters, got {}.",
            uid.len()
        )));
    }

    // Validate quality if provided
    if let Some(q) = quality {
        if q < 1 || q > 7 {
            return Err(McpError::invalid_request(format!(
                "Invalid quality value: {}. Quality must be between 1 and 7.",
                q
            )));
        }
    }

    // Build update object
    let updates = PhotoUpdate {
        title,
        description,
        favorite,
        private,
        quality,
        keywords,
        lat,
        lng,
        altitude,
        country,
        place,
    };

    ctx.info(&format!("Updating photo: {}", uid)).await?;

    let photo = client
        .update_photo(&uid, &updates)
        .await
        .map_err(|e| McpError::internal(format!("Photo update failed: {}", e)))?;

    ctx.info(&format!("Photo updated successfully: '{}'", photo.title))
        .await?;

    serde_json::to_string_pretty(&photo)
        .map_err(|e| McpError::internal(format!("Failed to serialize photo: {}", e)))
}

/// Delete a photo permanently from PhotoPrism.
///
/// **WARNING**: This operation is irreversible. The photo and all associated files
/// will be permanently removed. Consider archiving photos instead of deleting them
/// if you might want to restore them later.
///
/// # When to Use
///
/// - User explicitly requests deletion ("delete photo xyz")
/// - User confirms they want to permanently remove a photo
/// - User says "remove permanently" or "delete forever"
///
/// # When NOT to Use
///
/// - User just wants to hide photos → use archive instead
/// - User wants to remove from an album → use remove_from_album instead
/// - User hasn't confirmed deletion → ask for confirmation first
///
/// # Examples
///
/// User: "Delete photo abc123def4567890"
/// → delete_photo(uid="abc123def4567890")
///
/// User: "Permanently remove that blurry photo"
/// → delete_photo(uid="<uid_from_search>")
///
/// # Best Practice
///
/// Before deleting, consider:
/// 1. Confirming with user that they want permanent deletion
/// 2. Showing photo details so they can verify it's the right one
/// 3. Suggesting archiving as an alternative
///
/// # Parameters
///
/// - `uid`: Photo unique identifier (16 characters, required)
///
/// # Returns
///
/// Success message confirming deletion.
///
/// # Errors
///
/// - Returns error if photo not found
/// - Returns error if UID is invalid
/// - Returns error if deletion fails (e.g., permission denied)
#[allow(dead_code)]
pub async fn delete_photo_tool(
    client: &PhotoPrismClient,
    ctx: Context,
    uid: String,
) -> McpResult<String> {
    // Validate UID format
    if uid.is_empty() {
        return Err(McpError::invalid_request(
            "Photo UID cannot be empty. Please provide a 16-character photo identifier.",
        ));
    }

    if uid.len() != 16 {
        return Err(McpError::invalid_request(format!(
            "Invalid UID format: expected 16 characters, got {}.",
            uid.len()
        )));
    }

    ctx.warn(&format!("Deleting photo (PERMANENT): {}", uid))
        .await?;

    client
        .delete_photo(&uid)
        .await
        .map_err(|e| McpError::internal(format!("Photo deletion failed: {}", e)))?;

    ctx.info(&format!("Photo {} deleted successfully", uid))
        .await?;

    Ok(format!(
        "Photo {} has been permanently deleted from PhotoPrism.",
        uid
    ))
}

/// Archive multiple photos at once.
///
/// Archives photos so they're hidden from the main library but can be restored later.
/// This is safer than permanent deletion and recommended for photos you might want
/// to recover in the future.
///
/// # When to Use
///
/// - User wants to hide multiple photos without deleting
/// - User says "archive these photos"
/// - User wants to clean up library but keep files
/// - User wants to remove photos from view temporarily
///
/// # Examples
///
/// User: "Archive these 5 photos"
/// → archive_photos(uids=["uid1", "uid2", "uid3", "uid4", "uid5"])
///
/// User: "Hide all blurry photos from the search results"
/// → (after searching) archive_photos(uids=[...])
///
/// # Parameters
///
/// - `uids`: Array of photo UIDs to archive (at least 1 required)
///
/// # Returns
///
/// Summary of operation including success count and any errors.
#[allow(dead_code)]
pub async fn archive_photos_tool(
    client: &PhotoPrismClient,
    ctx: Context,
    uids: Vec<String>,
) -> McpResult<String> {
    if uids.is_empty() {
        return Err(McpError::invalid_request(
            "At least one photo UID is required for batch archiving.",
        ));
    }

    ctx.info(&format!("Archiving {} photos", uids.len()))
        .await?;

    let result = client
        .archive_photos(uids)
        .await
        .map_err(|e| McpError::internal(format!("Batch archive failed: {}", e)))?;

    ctx.info(&format!(
        "Archive complete: {} succeeded, {} failed",
        result.count, result.errors
    ))
    .await?;

    serde_json::to_string_pretty(&result)
        .map_err(|e| McpError::internal(format!("Failed to serialize result: {}", e)))
}

/// Restore multiple archived photos.
///
/// Restores previously archived photos back to the main library, making them
/// visible again.
///
/// # When to Use
///
/// - User wants to un-archive photos
/// - User says "restore these photos"
/// - User wants to bring back archived photos
///
/// # Parameters
///
/// - `uids`: Array of photo UIDs to restore (at least 1 required)
///
/// # Returns
///
/// Summary of operation including success count and any errors.
#[allow(dead_code)]
pub async fn restore_photos_tool(
    client: &PhotoPrismClient,
    ctx: Context,
    uids: Vec<String>,
) -> McpResult<String> {
    if uids.is_empty() {
        return Err(McpError::invalid_request(
            "At least one photo UID is required for batch restore.",
        ));
    }

    ctx.info(&format!("Restoring {} photos", uids.len()))
        .await?;

    let result = client
        .restore_photos(uids)
        .await
        .map_err(|e| McpError::internal(format!("Batch restore failed: {}", e)))?;

    ctx.info(&format!(
        "Restore complete: {} succeeded, {} failed",
        result.count, result.errors
    ))
    .await?;

    serde_json::to_string_pretty(&result)
        .map_err(|e| McpError::internal(format!("Failed to serialize result: {}", e)))
}

/// Mark multiple photos as favorites.
///
/// Batch operation to add favorite status to multiple photos at once.
/// Favorited photos can be easily filtered and accessed.
///
/// # When to Use
///
/// - User wants to favorite multiple photos at once
/// - User says "mark all these as favorites"
/// - User wants to save best photos from a set
///
/// # Examples
///
/// User: "Mark all sunset photos from yesterday as favorites"
/// → (after search) like_photos(uids=[...])
///
/// # Parameters
///
/// - `uids`: Array of photo UIDs to favorite (at least 1 required)
///
/// # Returns
///
/// Summary of operation including success count and any errors.
#[allow(dead_code)]
pub async fn like_photos_tool(
    client: &PhotoPrismClient,
    ctx: Context,
    uids: Vec<String>,
) -> McpResult<String> {
    if uids.is_empty() {
        return Err(McpError::invalid_request(
            "At least one photo UID is required for batch favorite.",
        ));
    }

    ctx.info(&format!("Adding {} photos to favorites", uids.len()))
        .await?;

    let result = client
        .like_photos(uids)
        .await
        .map_err(|e| McpError::internal(format!("Batch favorite failed: {}", e)))?;

    ctx.info(&format!(
        "Favorite complete: {} succeeded, {} failed",
        result.count, result.errors
    ))
    .await?;

    serde_json::to_string_pretty(&result)
        .map_err(|e| McpError::internal(format!("Failed to serialize result: {}", e)))
}

/// Remove favorite status from multiple photos.
///
/// Batch operation to remove favorite status from multiple photos at once.
///
/// # When to Use
///
/// - User wants to unfavorite multiple photos
/// - User says "remove these from favorites"
/// - User wants to clean up favorite collection
///
/// # Parameters
///
/// - `uids`: Array of photo UIDs to unfavorite (at least 1 required)
///
/// # Returns
///
/// Summary of operation including success count and any errors.
#[allow(dead_code)]
pub async fn dislike_photos_tool(
    client: &PhotoPrismClient,
    ctx: Context,
    uids: Vec<String>,
) -> McpResult<String> {
    if uids.is_empty() {
        return Err(McpError::invalid_request(
            "At least one photo UID is required for batch unfavorite.",
        ));
    }

    ctx.info(&format!("Removing {} photos from favorites", uids.len()))
        .await?;

    let result = client
        .dislike_photos(uids)
        .await
        .map_err(|e| McpError::internal(format!("Batch unfavorite failed: {}", e)))?;

    ctx.info(&format!(
        "Unfavorite complete: {} succeeded, {} failed",
        result.count, result.errors
    ))
    .await?;

    serde_json::to_string_pretty(&result)
        .map_err(|e| McpError::internal(format!("Failed to serialize result: {}", e)))
}

// ============================================================================
// Integration Examples
// ============================================================================

/// Example of how these tools would be registered in the PhotoPrismServer.
///
/// In the actual implementation (src/server/photoprism.rs), you would use:
///
/// ```rust,ignore
/// #[turbomcp::server(
///     name = "photoprism",
///     version = "0.1.0",
///     description = "MCP server for PhotoPrism photo library management",
/// )]
/// impl PhotoPrismServer {
///     /// Search photos using simple text query
///     #[tool("Search photos by text query. Use for basic searches without filters.")]
///     async fn search_photos(
///         &self,
///         ctx: Context,
///         #[description("Search query (can be empty to get all photos)")] query: String,
///         #[description("Maximum number of results (default: 100, max: 1000)")] count: Option<u32>,
///     ) -> McpResult<String> {
///         search_photos_tool(&self.client, ctx, query, count).await
///     }
///
///     /// Search photos with advanced filters
///     #[tool("Search photos with advanced filters including type, quality, location, date, and more")]
///     async fn search_photos_advanced(
///         &self,
///         ctx: Context,
///         params: PhotoSearchParams,
///     ) -> McpResult<String> {
///         search_photos_advanced_tool(&self.client, ctx, params).await
///     }
///
///     // ... etc for other tools
/// }
/// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_documentation_exists() {
        // Verify that all tools have proper documentation
        // This is a placeholder - actual tests would verify tool registration
        assert!(true);
    }
}
