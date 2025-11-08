# Safe Batch Operation Patterns for PhotoPrism MCP Server

## Overview

Batch operations allow processing multiple photos or albums in a single MCP tool invocation. This document outlines best practices and safety patterns implemented in the PhotoPrism MCP server to ensure reliable, safe, and user-friendly batch processing.

## Design Principles

### 1. **Safety First**

Batch operations can affect many items at once, making safety paramount:

- **Confirmation Required**: Destructive operations (like delete) require explicit confirmation
- **Limited Batch Sizes**: Maximum limits prevent accidental mass operations
- **Clear Warnings**: Operations log warnings for destructive actions
- **No Silent Failures**: All errors are reported with context

### 2. **Partial Success Handling**

Unlike all-or-nothing transactions, batch operations use a "continue-on-error" model:

- Each item is processed independently
- Failure of one item doesn't stop processing others
- Detailed per-item results are returned
- Summary statistics show overall success rate

### 3. **Idempotency**

Batch operations are designed to be safely retryable:

- Operations can be repeated without unintended side effects
- Archive/unarchive: Archiving an already-archived photo is safe
- Favorite/unfavorite: Setting favorite status multiple times is safe
- Add to album: Adding a photo already in an album is handled gracefully

### 4. **Transparency**

Users receive complete information about what happened:

- Summary statistics (total, success_count, error_count, skipped_count)
- Detailed per-item results
- Clear error messages for failures
- Progress logging during execution

## Batch Operation Limits

Different operations have different risk profiles and appropriate limits:

| Operation | Max Batch Size | Rationale |
|-----------|---------------|-----------|
| Archive/Restore | 100 | Low risk, reversible |
| Favorite/Unfavorite | 100 | Low risk, reversible |
| Make Private/Public | 100 | Medium risk, reversible |
| Update Metadata | 50 | Medium risk, harder to undo |
| Delete | 50 | High risk, permanent |
| Add to Album | 100 | Low risk, reversible |
| Remove from Album | 100 | Low risk, reversible |

## Implementation Patterns

### Pattern 1: Basic Batch Operation

```rust
pub async fn batch_operation(
    &self,
    ctx: Context,
    photo_uids: Vec<String>,
) -> McpResult<String> {
    // 1. Validate inputs
    if photo_uids.is_empty() {
        return Err(McpError::invalid_request("At least one photo UID required"));
    }

    if photo_uids.len() > MAX_BATCH_SIZE {
        return Err(McpError::invalid_request(
            format!("Maximum {} photos can be processed at once", MAX_BATCH_SIZE)
        ));
    }

    // 2. Log start
    ctx.info(&format!("Processing {} photos...", photo_uids.len())).await?;

    // 3. Process each item
    let mut results = Vec::new();
    for uid in photo_uids {
        match self.process_single_item(&uid).await {
            Ok(_) => {
                results.push(BatchItemResult::success(uid, None));
            }
            Err(e) => {
                results.push(BatchItemResult::error(uid, None, e.to_string()));
            }
        }
    }

    // 4. Build response
    let response = BatchOperationResponse::from_results(
        results,
        Some("Batch operation completed".to_string()),
    );

    // 5. Log summary
    ctx.info(&format!(
        "Complete: {} succeeded, {} failed",
        response.summary.success_count,
        response.summary.error_count
    )).await?;

    Ok(serde_json::to_string_pretty(&response)?)
}
```

### Pattern 2: Destructive Operation with Confirmation

```rust
pub async fn batch_delete_photos(
    &self,
    ctx: Context,
    photo_uids: Vec<String>,
    confirm: bool,
    permanent: Option<bool>,
) -> McpResult<String> {
    // 1. Require explicit confirmation
    if !confirm {
        return Err(McpError::invalid_request(
            "Deletion not confirmed. Set confirm=true to proceed. WARNING: Permanent!"
        ));
    }

    // 2. Stricter limits for destructive operations
    if photo_uids.len() > 50 {
        return Err(McpError::invalid_request(
            "Maximum 50 photos can be deleted at once for safety"
        ));
    }

    // 3. Clear warning logging
    ctx.warn(&format!(
        "⚠️  {} deleting {} photos...",
        if permanent.unwrap_or(false) { "Permanently" } else { "Soft" },
        photo_uids.len()
    )).await?;

    // ... process items ...
}
```

### Pattern 3: Operation with Optional Parameters

```rust
pub async fn batch_update_photos(
    &self,
    ctx: Context,
    photo_uids: Vec<String>,
    title: Option<String>,
    description: Option<String>,
    add_tags: Option<Vec<String>>,
) -> McpResult<String> {
    // Validate at least one update parameter provided
    if title.is_none() && description.is_none() && add_tags.is_none() {
        return Err(McpError::invalid_request(
            "At least one update parameter must be provided"
        ));
    }

    // ... process items ...
}
```

## Response Structure

All batch operations return a consistent response format:

```json
{
  "summary": {
    "total": 10,
    "success_count": 8,
    "error_count": 2,
    "skipped_count": 0,
    "success_rate": 0.8
  },
  "results": [
    {
      "id": "abc123xyz456",
      "name": "sunset.jpg",
      "status": "success",
      "details": null,
      "error": null
    },
    {
      "id": "def456uvw789",
      "name": "beach.jpg",
      "status": "error",
      "details": null,
      "error": "Photo not found: 404"
    }
  ],
  "message": "Batch archive operation completed"
}
```

### Response Fields

- **summary.total**: Total number of items processed
- **summary.success_count**: Number of successful operations
- **summary.error_count**: Number of failed operations
- **summary.skipped_count**: Number of items skipped (validation failed)
- **summary.success_rate**: Ratio of successes (0.0 to 1.0)
- **results[]**: Array of per-item results
- **results[].id**: Item identifier (photo UID)
- **results[].name**: Item name (if available, for readability)
- **results[].status**: "success", "error", or "skipped"
- **results[].error**: Error message if status is "error"
- **results[].details**: Additional information (e.g., skip reason)
- **message**: Overall operation description

## Error Handling Strategies

### Per-Item Error Handling

```rust
for uid in photo_uids {
    match self.process_item(&uid).await {
        Ok(_) => {
            ctx.info(&format!("Successfully processed: {}", uid)).await?;
            results.push(BatchItemResult::success(uid, None));
        }
        Err(e) => {
            // Log error but continue processing
            ctx.error(&format!("Error for {}: {}", uid, e)).await?;
            results.push(BatchItemResult::error(
                uid,
                None,
                e.to_string()
            ));
        }
    }
}
```

### Validation Errors

```rust
// Validate before processing
for uid in &photo_uids {
    if uid.len() != 16 {
        results.push(BatchItemResult::skipped(
            uid.clone(),
            None,
            "Invalid UID format (must be 16 characters)".to_string()
        ));
        continue;
    }
    // ... process valid items ...
}
```

## LLM-Friendly Tool Design

### Clear Descriptions

Tools are designed with clear, descriptive documentation that helps LLMs understand:

```rust
#[tool("Archive or restore multiple photos in batch (max 100 photos per operation)")]
pub async fn batch_archive_photos(
    &self,
    ctx: Context,
    #[description("List of photo UIDs to archive (max 100)")]
    photo_uids: Vec<String>,
    #[description("Set to true to restore (unarchive) instead of archive")]
    restore: Option<bool>,
) -> McpResult<String>
```

### Workflow Recommendations

**Two-Phase Approach**: For complex operations, recommend a search-then-act pattern:

1. **Phase 1**: Use search tools to identify photos
   ```
   User: "Archive all photos from 2020"
   LLM: Uses search_photos with year=2020 filter
   ```

2. **Phase 2**: Use batch operation with discovered UIDs
   ```
   LLM: Uses batch_archive_photos with UIDs from search
   ```

**Progressive Operations**: For large datasets, recommend batching:

```
User: "Delete 500 photos"
LLM Response: "I can delete photos in batches of 50 for safety.
             Let me process the first batch, then continue with your approval."
```

## Common Workflows

### Workflow 1: Archive Old Photos

```
1. search_photos(query="year:2019", count=100)
2. Review results
3. batch_archive_photos(photo_uids=[...], restore=false)
```

### Workflow 2: Organize into Albums

```
1. create_album(title="Vacation 2024")
2. search_photos(query="location:Hawaii date:2024")
3. add_photos_to_album(album_uid="...", photo_uids=[...])
```

### Workflow 3: Bulk Favorite Best Photos

```
1. search_photos(query="quality:5", count=50)
2. batch_favorite_photos(photo_uids=[...], unfavorite=false)
```

### Workflow 4: Clean Up Duplicates

```
1. search_photos(query="duplicate:true")
2. Review duplicate sets
3. batch_delete_photos(photo_uids=[...], confirm=true, permanent=false)
```

## Safety Checklist for Batch Operations

When implementing new batch operations, ensure:

- [ ] Input validation (non-empty, within limits)
- [ ] Appropriate batch size limits for operation risk level
- [ ] Confirmation required for destructive operations
- [ ] Clear warning logs for dangerous operations
- [ ] Per-item error handling (continue on failure)
- [ ] Detailed result reporting
- [ ] Summary statistics
- [ ] Idempotent design (safe to retry)
- [ ] Clear tool descriptions for LLMs
- [ ] Progress logging
- [ ] Error context (which item, what error)

## Advanced Patterns

### Rate Limiting

For API rate limits, implement delays:

```rust
for (index, uid) in photo_uids.iter().enumerate() {
    // Process item

    // Small delay every 10 items to avoid rate limits
    if index > 0 && index % 10 == 0 {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
```

### Progress Reporting

For long-running operations:

```rust
let total = photo_uids.len();
for (index, uid) in photo_uids.iter().enumerate() {
    // Process item

    // Report progress every 25%
    if index > 0 && index % (total / 4) == 0 {
        ctx.info(&format!(
            "Progress: {}/{} photos processed ({}%)",
            index,
            total,
            (index * 100) / total
        )).await?;
    }
}
```

### Dry Run Mode

Consider adding dry-run capability:

```rust
pub async fn batch_delete_photos(
    &self,
    ctx: Context,
    photo_uids: Vec<String>,
    confirm: bool,
    dry_run: Option<bool>,
) -> McpResult<String> {
    if dry_run.unwrap_or(false) {
        return Ok(format!(
            "DRY RUN: Would delete {} photos: {:?}",
            photo_uids.len(),
            photo_uids
        ));
    }
    // ... actual delete ...
}
```

## Testing Strategies

### Unit Tests

Test individual batch logic:

```rust
#[tokio::test]
async fn test_batch_archive_empty_list() {
    let result = batch_archive_photos(vec![]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("at least one"));
}

#[tokio::test]
async fn test_batch_archive_exceeds_limit() {
    let uids: Vec<String> = (0..101).map(|i| format!("uid{}", i)).collect();
    let result = batch_archive_photos(uids);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Maximum"));
}
```

### Integration Tests

Test with mock API:

```rust
#[tokio::test]
async fn test_batch_partial_failure() {
    let mut server = mockito::Server::new();

    // Mock success for first photo
    server.mock("POST", "/api/v1/photos/uid1/archive")
        .with_status(200)
        .create();

    // Mock failure for second photo
    server.mock("POST", "/api/v1/photos/uid2/archive")
        .with_status(404)
        .create();

    let result = batch_archive_photos(vec!["uid1", "uid2"]);
    assert!(result.is_ok());

    let response: BatchOperationResponse = serde_json::from_str(&result.unwrap())?;
    assert_eq!(response.summary.success_count, 1);
    assert_eq!(response.summary.error_count, 1);
}
```

## Conclusion

Safe batch operations are critical for a production-ready MCP server. By following these patterns, we ensure:

1. **User Safety**: Confirmations, limits, and warnings prevent accidents
2. **Reliability**: Per-item error handling ensures partial success
3. **Transparency**: Detailed results inform users of what happened
4. **LLM-Friendly**: Clear descriptions enable effective AI assistance
5. **Maintainability**: Consistent patterns make code easy to understand

These patterns form the foundation for building robust, safe, and user-friendly batch operations in the PhotoPrism MCP server.
