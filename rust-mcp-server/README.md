# PhotoPrism MCP Server - Album & Batch Operations

A comprehensive Rust implementation of Album Management and Batch Operation tools for the PhotoPrism Model Context Protocol (MCP) server.

## Overview

This implementation provides a robust, safe, and LLM-friendly interface for managing PhotoPrism albums and performing batch operations on photos. Built with TurboMCP 2.2.1, it emphasizes type safety, error handling, and user-friendly workflows.

## Features

### Album Management Tools (8 tools)

1. **list_albums** - Search and filter albums with pagination
2. **get_album** - Retrieve detailed album information
3. **create_album** - Create new albums with metadata
4. **update_album** - Modify album properties
5. **delete_album** - Remove albums (with confirmation)
6. **add_photos_to_album** - Add photos to albums (max 100)
7. **remove_photos_from_album** - Remove photos from albums (max 100)
8. **toggle_album_favorite** - Mark albums as favorite/unfavorite
9. **clone_album** - Create copies of existing albums

### Batch Operation Tools (5 tools)

1. **batch_archive_photos** - Archive/restore multiple photos (max 100)
2. **batch_delete_photos** - Delete multiple photos with safety checks (max 50)
3. **batch_favorite_photos** - Mark photos as favorite/unfavorite (max 100)
4. **batch_private_photos** - Set privacy status for photos (max 100)
5. **batch_update_photos** - Update metadata for multiple photos (max 50)

## Architecture

```
rust-mcp-server/
├── src/
│   ├── types/
│   │   ├── album.rs          # Album data structures
│   │   ├── batch.rs          # Batch operation types
│   │   └── mod.rs
│   ├── client/
│   │   ├── albums.rs         # Album API methods
│   │   └── mod.rs            # PhotoPrismClient
│   ├── tools/
│   │   ├── albums.rs         # Album MCP tools
│   │   ├── batch.rs          # Batch MCP tools
│   │   └── mod.rs
│   ├── server/               # (To be implemented)
│   └── main.rs               # (To be implemented)
├── Cargo.toml
├── README.md
├── BATCH_OPERATIONS.md       # Safe batch patterns documentation
└── WORKFLOW_EXAMPLES.md      # LLM workflow examples
```

## Key Design Principles

### 1. Safety First

- **Confirmation Required**: Destructive operations require explicit `confirm=true`
- **Batch Size Limits**: Prevents accidental mass operations
  - Delete: max 50 photos (high risk)
  - Update: max 50 photos (medium risk)
  - Archive/Favorite: max 100 photos (low risk)
- **Clear Warnings**: Logs warnings for dangerous operations
- **Soft Delete Default**: Delete moves to trash by default, not permanent deletion

### 2. Partial Success Handling

- **Continue on Error**: Individual item failures don't stop batch processing
- **Detailed Results**: Every item gets success/error/skipped status
- **Summary Statistics**: Overall success rate and counts
- **Error Context**: Clear error messages with item identifiers

### 3. Idempotent Operations

All operations are designed to be safely retryable:

- Adding photos to albums handles duplicates gracefully
- Archiving already-archived photos succeeds
- Favoriting already-favorite photos succeeds
- Safe to retry failed batches without duplicating successful items

### 4. LLM-Friendly Design

- **Clear Descriptions**: Comprehensive tool documentation
- **JSON Schema**: Automatic validation from Rust types
- **Structured Responses**: Consistent JSON output format
- **Progress Logging**: Context-aware logging for transparency

## Type System

### Album Types

```rust
pub struct Album {
    pub uid: String,
    pub title: String,
    pub album_type: AlbumType,
    pub photo_count: u32,
    pub favorite: bool,
    pub private: bool,
    // ... 20+ fields total
}

pub struct AlbumCreateParams {
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub favorite: bool,
    pub private: bool,
}

pub struct AlbumSearchParams {
    pub q: Option<String>,
    pub album_type: Option<AlbumType>,
    pub category: Option<String>,
    pub count: u32,
    pub offset: u32,
    // ... filters
}
```

### Batch Types

```rust
pub struct BatchOperationResponse {
    pub summary: BatchSummary,
    pub results: Vec<BatchItemResult>,
    pub message: Option<String>,
}

pub struct BatchSummary {
    pub total: usize,
    pub success_count: usize,
    pub error_count: usize,
    pub skipped_count: usize,
    pub success_rate: f64,
}

pub struct BatchItemResult {
    pub id: String,
    pub name: Option<String>,
    pub status: BatchItemStatus, // Success, Error, Skipped
    pub error: Option<String>,
    pub details: Option<String>,
}
```

## API Client Methods

The `PhotoPrismClient` provides:

```rust
// Album operations
async fn list_albums(&self, params: Option<AlbumSearchParams>) -> Result<Vec<Album>>
async fn get_album(&self, uid: &str) -> Result<Album>
async fn create_album(&self, params: AlbumCreateParams) -> Result<Album>
async fn update_album(&self, uid: &str, params: AlbumUpdateParams) -> Result<Album>
async fn delete_album(&self, uid: &str) -> Result<()>

// Album-photo operations
async fn add_photos_to_album(&self, album_uid: &str, photo_uids: Vec<String>) -> Result<Album>
async fn remove_photos_from_album(&self, album_uid: &str, photo_uids: Vec<String>) -> Result<()>

// Album actions
async fn like_album(&self, uid: &str) -> Result<Album>
async fn unlike_album(&self, uid: &str) -> Result<Album>
async fn clone_album(&self, uid: &str) -> Result<Album>
async fn download_album(&self, uid: &str) -> Result<Vec<u8>>
```

## Usage Examples

### Creating and Populating an Album

```rust
// Create album
let album = client.create_album(AlbumCreateParams {
    title: "Vacation 2024".to_string(),
    description: Some("Summer vacation photos".to_string()),
    category: Some("Travel".to_string()),
    favorite: true,
    private: false,
    location: None,
}).await?;

// Add photos
let photo_uids = vec!["photo1uid".to_string(), "photo2uid".to_string()];
client.add_photos_to_album(&album.uid, photo_uids).await?;
```

### Batch Archive Operation

```rust
let photo_uids: Vec<String> = /* ... */;

let response = batch_tools.batch_archive_photos(
    ctx,
    photo_uids,
    Some(false) // archive (not restore)
).await?;

// Response includes:
// - summary.total: Total photos processed
// - summary.success_count: Successfully archived
// - summary.error_count: Failed operations
// - results: Per-photo details
```

### Safe Batch Delete

```rust
let response = batch_tools.batch_delete_photos(
    ctx,
    photo_uids,
    true,              // confirm=true (required!)
    Some(false)        // permanent=false (soft delete)
).await?;

// Only proceeds if confirm=true
// Max 50 photos for safety
// Soft delete by default (recoverable)
```

## Batch Operation Response Format

All batch operations return a consistent JSON structure:

```json
{
  "summary": {
    "total": 100,
    "success_count": 97,
    "error_count": 3,
    "skipped_count": 0,
    "success_rate": 0.97
  },
  "results": [
    {
      "id": "abc123xyz456",
      "name": "sunset.jpg",
      "status": "success",
      "error": null,
      "details": null
    },
    {
      "id": "def456uvw789",
      "name": "beach.jpg",
      "status": "error",
      "error": "Photo not found: 404",
      "details": null
    }
  ],
  "message": "Batch archive operation completed"
}
```

## Error Handling

### Input Validation

```rust
// Empty list
if photo_uids.is_empty() {
    return Err(McpError::invalid_request("At least one photo UID required"));
}

// Batch size limits
if photo_uids.len() > 50 {
    return Err(McpError::invalid_request("Maximum 50 photos allowed"));
}

// UID format
if uid.len() != 16 {
    return Err(McpError::invalid_request("UID must be 16 characters"));
}
```

### Per-Item Error Handling

```rust
for uid in photo_uids {
    match self.process_photo(&uid).await {
        Ok(_) => {
            results.push(BatchItemResult::success(uid, None));
        }
        Err(e) => {
            // Log but continue processing
            ctx.error(&format!("Error for {}: {}", uid, e)).await?;
            results.push(BatchItemResult::error(uid, None, e.to_string()));
        }
    }
}
```

## Common Workflows

### Workflow 1: Event Organization

```
1. create_album(title="Wedding - Ceremony")
2. create_album(title="Wedding - Reception")
3. search_photos(query="wedding ceremony")
4. add_photos_to_album(album_uid, photo_uids)
5. search_photos(query="wedding reception")
6. add_photos_to_album(album_uid, photo_uids)
```

### Workflow 2: Privacy Batch Update

```
1. search_photos(query="location:home")
2. batch_private_photos(photo_uids, make_public=false)
3. Verify 200+ photos now private
```

### Workflow 3: Quality-Based Favorites

```
1. search_photos(query="quality:5")
2. batch_favorite_photos(photo_uids, unfavorite=false)
3. create_album(title="Best Photos", favorite=true)
4. add_photos_to_album(album_uid, photo_uids)
```

See [WORKFLOW_EXAMPLES.md](./WORKFLOW_EXAMPLES.md) for detailed workflow patterns.

## Testing

### Unit Tests

```rust
#[tokio::test]
async fn test_batch_size_validation() {
    let uids: Vec<String> = (0..101).map(|i| format!("uid{}", i)).collect();
    let result = batch_archive_photos(uids);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Maximum"));
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_partial_batch_failure() {
    // Mock API with mixed success/failure
    let result = batch_archive_photos(vec!["valid_uid", "invalid_uid"]).await?;
    let response: BatchOperationResponse = serde_json::from_str(&result)?;

    assert_eq!(response.summary.total, 2);
    assert_eq!(response.summary.success_count, 1);
    assert_eq!(response.summary.error_count, 1);
}
```

## Documentation

- **[IMPLEMENTATION_GUIDE.md](../IMPLEMENTATION_GUIDE.md)** - Overall MCP server architecture
- **[BATCH_OPERATIONS.md](./BATCH_OPERATIONS.md)** - Safe batch operation patterns
- **[WORKFLOW_EXAMPLES.md](./WORKFLOW_EXAMPLES.md)** - Practical workflow examples

## Dependencies

```toml
[dependencies]
turbomcp = "2.2.1"           # MCP framework
tokio = "1.47"                # Async runtime
reqwest = "0.12"              # HTTP client
serde = "1.0"                 # Serialization
schemars = "1.0"              # JSON Schema
anyhow = "1.0"                # Error handling
thiserror = "2.0"             # Custom errors
chrono = "0.4"                # Date/time
urlencoding = "2.1"           # URL encoding
```

## Implementation Quality Criteria

✅ **Type Safety**: All types derive `JsonSchema` for automatic validation

✅ **Error Handling**: Comprehensive error handling with clear messages

✅ **Batch Safety**: Size limits, confirmations, and partial failure handling

✅ **Idempotency**: All operations safe to retry

✅ **Documentation**: Clear tool descriptions for LLM understanding

✅ **Testing**: Unit and integration test patterns defined

✅ **Logging**: Context-aware progress and error logging

✅ **Validation**: Input validation at multiple levels

## Next Steps

To complete the implementation:

1. **Server Setup**: Implement `src/server/mod.rs` and `src/main.rs`
2. **Resource Handlers**: Add read-only resource endpoints
3. **Photo Tools**: Implement photo search and management tools
4. **Configuration**: Add config loading and environment setup
5. **Integration Tests**: Build comprehensive test suite
6. **Docker**: Create Docker image for deployment

## Research Summary

This implementation is informed by:

- **Batch Operation Patterns**: 207 Multi-Status responses, partial success handling, idempotency keys
- **REST API Best Practices**: Resource-specific endpoints, validation, clear error messages
- **MCP Design**: Model-controlled tools, bulk operation limits (max 20-100 items), structured responses
- **Rust Patterns**: Result/Option error handling, custom error types with thiserror, batch processing with continue-on-error

## License

See parent repository LICENSE file.

## Contributing

This implementation follows the patterns established in the PhotoPrism MCP Server Implementation Guide. Contributions should maintain:

- Type safety and compile-time validation
- Clear error messages and handling
- LLM-friendly tool descriptions
- Comprehensive documentation
- Safe batch operation patterns

## Acknowledgments

- PhotoPrism team for the excellent photo management platform
- TurboMCP for the zero-boilerplate MCP framework
- Anthropic for the Model Context Protocol specification
- Letta MCP Server for batch operation pattern inspiration
