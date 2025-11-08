# PhotoPrism MCP Server - Workflow Examples

This document provides practical examples of how Claude (or other LLM clients) can effectively use the PhotoPrism MCP server tools to accomplish common photo management tasks.

## Table of Contents

1. [Album Management Workflows](#album-management-workflows)
2. [Batch Operation Workflows](#batch-operation-workflows)
3. [Complex Multi-Step Workflows](#complex-multi-step-workflows)
4. [Error Recovery Workflows](#error-recovery-workflows)
5. [LLM Interaction Patterns](#llm-interaction-patterns)

---

## Album Management Workflows

### Workflow 1: Create Album and Add Photos

**User Request**: "Create a vacation album and add my photos from Hawaii"

**LLM Steps**:

```
1. list_albums(query="vacation")
   → Check if similar album exists

2. create_album(
     title="Hawaii Vacation 2024",
     description="Family vacation in Hawaii, Summer 2024",
     category="Travel",
     location="Hawaii",
     favorite=true
   )
   → Returns: { "uid": "abc123xyz456", "title": "Hawaii Vacation 2024", ... }

3. search_photos(query="location:Hawaii", count=100)
   → Returns list of photo UIDs matching Hawaii location

4. add_photos_to_album(
     album_uid="abc123xyz456",
     photo_uids=["photo1uid", "photo2uid", ...]
   )
   → Returns updated album with PhotoCount
```

**Success Indicators**:
- Album created with specific UID
- Photos successfully added (PhotoCount updated)
- All operations completed without errors

---

### Workflow 2: Organize Photos by Year

**User Request**: "Organize all my 2023 photos into a year album"

**LLM Steps**:

```
1. list_albums(album_type="month")
   → Check existing organization

2. create_album(
     title="2023 Highlights",
     description="Best photos from 2023",
     category="Year Review"
   )
   → Returns album UID

3. search_photos(query="year:2023 quality:4-5", count=200)
   → Find high-quality photos from 2023

4. add_photos_to_album(
     album_uid="...",
     photo_uids=[...]
   )
   → Batch add photos to album
```

---

### Workflow 3: Clone and Modify Album

**User Request**: "Make a copy of my 'Best Photos' album and make it public"

**LLM Steps**:

```
1. list_albums(query="Best Photos")
   → Find the album UID

2. clone_album(uid="original_album_uid")
   → Returns: { "uid": "new_album_uid", "title": "Best Photos (Copy)", ... }

3. update_album(
     uid="new_album_uid",
     title="Best Photos - Public Share",
     private=false
   )
   → Make the album public
```

---

## Batch Operation Workflows

### Workflow 4: Archive Old Photos

**User Request**: "Archive all photos older than 2020"

**LLM Steps**:

```
1. search_photos(query="before:2020-01-01", count=500)
   → Returns list of old photos

2. LLM Response: "Found 347 photos from before 2020. Would you like to
   proceed with archiving them? I'll process them in batches of 100."

3. User confirms

4. batch_archive_photos(
     photo_uids=[first 100 UIDs],
     restore=false
   )
   → Returns: { summary: { total: 100, success_count: 100, ... }, results: [...] }

5. batch_archive_photos(
     photo_uids=[next 100 UIDs],
     restore=false
   )
   → Continue until all photos processed

6. LLM Response: "Successfully archived 347 photos:
   - Batch 1: 100/100 succeeded
   - Batch 2: 100/100 succeeded
   - Batch 3: 100/100 succeeded
   - Batch 4: 47/47 succeeded"
```

**Best Practices**:
- Show user total count before proceeding
- Process in manageable batches
- Report progress after each batch
- Handle partial failures gracefully

---

### Workflow 5: Bulk Favorite Selection

**User Request**: "Mark all photos with 5-star quality as favorites"

**LLM Steps**:

```
1. search_photos(query="quality:5", count=1000)
   → Find all 5-star photos
   → Returns: 87 photos

2. batch_favorite_photos(
     photo_uids=[all 87 UIDs],
     unfavorite=false
   )
   → Returns batch results

3. LLM analyzes response:
   {
     "summary": {
       "total": 87,
       "success_count": 85,
       "error_count": 2
     },
     "results": [...]
   }

4. LLM Response: "Successfully marked 85 out of 87 photos as favorites.
   Two photos had errors:
   - photo_xyz: Photo not found (may have been deleted)
   - photo_abc: Permission denied"
```

---

### Workflow 6: Clean Up with Confirmation

**User Request**: "Delete all blurry photos with quality 1"

**LLM Steps**:

```
1. search_photos(query="quality:1 type:image", count=100)
   → Find low-quality photos
   → Returns: 43 photos

2. LLM Response: "I found 43 photos with quality rating 1 (blurry/poor quality).
   Would you like me to delete them? This will move them to trash (they can
   be restored within 30 days)."

3. User confirms

4. batch_delete_photos(
     photo_uids=[...],
     confirm=true,
     permanent=false
   )
   → Soft delete (move to trash)

5. LLM analyzes results and reports:
   "Deleted 43 photos:
   - 41 successfully moved to trash
   - 2 errors:
     * photo_xyz: Already in trash
     * photo_abc: File locked by another process"
```

**Safety Considerations**:
- Always preview what will be deleted
- Request user confirmation
- Use soft delete by default
- Report any errors clearly

---

## Complex Multi-Step Workflows

### Workflow 7: Event Organization Pipeline

**User Request**: "Organize my wedding photos - create albums for ceremony, reception, and portraits"

**LLM Steps**:

```
1. search_photos(query="wedding 2024", count=500)
   → Find all wedding photos
   → Returns: 387 photos

2. create_album(
     title="Wedding 2024 - Ceremony",
     category="Events",
     location="Downtown Cathedral"
   )
   → Returns: ceremony_album_uid

3. create_album(
     title="Wedding 2024 - Reception",
     category="Events",
     location="Grand Hotel Ballroom"
   )
   → Returns: reception_album_uid

4. create_album(
     title="Wedding 2024 - Portraits",
     category="Events"
   )
   → Returns: portraits_album_uid

5. search_photos(query="wedding ceremony 2024", count=200)
   → Find ceremony photos
   → Returns: 143 photos

6. add_photos_to_album(
     album_uid=ceremony_album_uid,
     photo_uids=[first 100]
   )

7. add_photos_to_album(
     album_uid=ceremony_album_uid,
     photo_uids=[remaining 43]
   )

8. (Repeat for reception and portraits)

9. list_albums(query="wedding")
   → Verify all albums created

10. LLM Response: "I've organized your 387 wedding photos into 3 albums:
    - Wedding 2024 - Ceremony: 143 photos
    - Wedding 2024 - Reception: 198 photos
    - Wedding 2024 - Portraits: 46 photos"
```

---

### Workflow 8: Privacy Batch Update

**User Request**: "Make all photos from my home address private"

**LLM Steps**:

```
1. search_photos(
     query="location:123 Main St, Anytown",
     count=1000
   )
   → Returns: 234 photos

2. LLM Response: "Found 234 photos taken at your home address.
   I'll make them all private in batches of 100."

3. batch_private_photos(
     photo_uids=[first 100],
     make_public=false
   )

4. batch_private_photos(
     photo_uids=[next 100],
     make_public=false
   )

5. batch_private_photos(
     photo_uids=[remaining 34],
     make_public=false
   )

6. LLM Response: "Successfully made 234 photos private:
   - Batch 1: 100/100 ✓
   - Batch 2: 100/100 ✓
   - Batch 3: 34/34 ✓"
```

---

### Workflow 9: Smart Album with Filtering

**User Request**: "Create a 'Best of 2024' album with only 5-star favorites"

**LLM Steps**:

```
1. create_album(
     title="Best of 2024",
     description="Highest quality favorite photos from 2024",
     favorite=true
   )
   → Returns: album_uid

2. search_photos(
     query="year:2024 quality:5 favorite:true",
     count=500
   )
   → Returns: 67 matching photos

3. add_photos_to_album(
     album_uid=album_uid,
     photo_uids=[all 67 UIDs]
   )

4. get_album(uid=album_uid)
   → Verify final state

5. LLM Response: "Created 'Best of 2024' album with 67 photos:
   - All 5-star quality
   - All marked as favorites
   - All from 2024
   - Album itself marked as favorite for quick access"
```

---

## Error Recovery Workflows

### Workflow 10: Handling Partial Batch Failures

**User Request**: "Add 150 photos to my vacation album"

**LLM Steps**:

```
1. add_photos_to_album(
     album_uid="vacation_album",
     photo_uids=[first 100 UIDs]
   )
   → Success: All 100 added

2. add_photos_to_album(
     album_uid="vacation_album",
     photo_uids=[remaining 50 UIDs]
   )
   → Returns batch result with some errors:
   {
     "summary": {
       "total": 50,
       "success_count": 47,
       "error_count": 3
     },
     "results": [
       { "id": "photo1", "status": "success" },
       ...
       { "id": "photo48", "status": "error", "error": "Photo not found" },
       { "id": "photo49", "status": "error", "error": "Photo already in album" },
       { "id": "photo50", "status": "error", "error": "Invalid UID format" }
     ]
   }

3. LLM analyzes errors and responds:
   "Successfully added 147 out of 150 photos to your vacation album.

   3 photos had errors:
   - photo48: Photo not found (may have been deleted)
   - photo49: Photo was already in the album
   - photo50: Invalid UID format

   The album now contains 247 photos total."
```

**Error Handling Best Practices**:
- Always check batch operation responses
- Categorize errors (permanent vs retryable)
- Don't retry already-succeeded items
- Report clear status to user
- Offer remediation options if available

---

### Workflow 11: Idempotent Retry

**User Request**: "Make sure all my landscape photos are in the Landscapes album"

**LLM Steps**:

```
1. get_album(uid="landscapes_album")
   → Current PhotoCount: 234

2. search_photos(query="subject:landscape", count=500)
   → Found: 312 landscape photos

3. add_photos_to_album(
     album_uid="landscapes_album",
     photo_uids=[all 312 UIDs]
   )
   → PhotoPrism handles duplicates gracefully
   → Returns: Album with PhotoCount: 312

4. LLM Response: "All 312 landscape photos are now in the Landscapes album.
   - 78 photos were newly added
   - 234 photos were already in the album
   - Total album size: 312 photos"
```

**Why This Works**:
- Adding photos to an album is idempotent
- Photos already in album are silently skipped
- Safe to retry or re-run
- Final state is guaranteed correct

---

## LLM Interaction Patterns

### Pattern 1: Progressive Disclosure

For large operations, show progress incrementally:

```
LLM: "I found 847 photos matching your criteria. Processing in batches of 100..."

[After batch 1]
LLM: "Processed 100/847 (12%) - 100 succeeded ✓"

[After batch 2]
LLM: "Processed 200/847 (24%) - 200 succeeded ✓"

...

[After completion]
LLM: "Completed processing all 847 photos:
     - 8 batches processed
     - 841 succeeded (99.3%)
     - 6 failed (0.7%)
     - Details of failures: ..."
```

---

### Pattern 2: Confirmation for Destructive Actions

Always confirm before destructive operations:

```
User: "Delete all photos from 2010"

LLM: "I found 234 photos from 2010. Before deleting:
     - Deletion will move photos to trash (recoverable for 30 days)
     - You can permanently delete later if desired
     - Would you like to proceed?"

User: "Yes"

LLM: [Executes batch_delete_photos with confirm=true]
```

---

### Pattern 3: Smart Defaults

Use intelligent defaults based on context:

```
User: "Archive my old photos"

LLM: "I'll archive photos older than 2 years (before 2022).
     Found 456 photos. Should I:
     a) Archive all 456 photos
     b) Let you review them first
     c) Use a different date cutoff"

User: "Go ahead with option a"

LLM: [Proceeds with batch operation]
```

---

### Pattern 4: Error Explanation and Recovery

Explain errors in user-friendly terms:

```
LLM: "I encountered some issues while adding photos:

     ✓ Successfully added: 47 photos

     ⚠ Skipped (already in album): 3 photos
        - These photos were already in the album

     ✗ Failed: 2 photos
        - photo_abc: Not found (may have been deleted)
        - photo_xyz: Permission denied (private photo)

     Would you like me to:
     1. Continue with other operations
     2. Investigate the failed photos
     3. Try again with just the failed photos"
```

---

### Pattern 5: Batch Size Optimization

Automatically optimize batch sizes:

```
User: "Update metadata for 500 photos"

LLM: "I'll update 500 photos in batches of 50 (the maximum for update
     operations). This will take about 10 batches.

     Processing batch 1/10..."

[Processes with progress updates]

LLM: "Completed all 10 batches:
     - Total: 500 photos
     - Succeeded: 496 photos
     - Failed: 4 photos
     - Details: ..."
```

---

## Advanced Workflow Patterns

### Pattern 6: Conditional Workflows

**User Request**: "Clean up my library - archive low quality duplicates"

```
1. search_photos(query="duplicate:true", count=1000)
   → Find all duplicates

2. For each duplicate set:
   - Keep highest quality version
   - Archive lower quality versions

3. batch_archive_photos(
     photo_uids=[lower_quality_duplicates]
   )

4. LLM Response: "Library cleanup complete:
   - Found 234 duplicate sets
   - Kept 234 best versions
   - Archived 412 lower quality duplicates
   - Freed up approximately 2.3 GB"
```

---

### Pattern 7: Workflow Templates

Common operations saved as templates:

**Template: "Monthly Photo Review"**
```
1. search_photos(query="month:current quality:4-5")
2. create_album(title="[Month] [Year] Highlights")
3. add_photos_to_album(...)
4. toggle_album_favorite(favorite=true)
5. Generate summary report
```

---

## Summary

These workflow examples demonstrate:

1. **Safety**: Always confirm destructive operations
2. **Progress**: Show incremental updates for long operations
3. **Error Handling**: Gracefully handle partial failures
4. **Idempotency**: Design for safe retries
5. **User Experience**: Clear communication at every step
6. **Batch Optimization**: Respect API limits and safety constraints
7. **Recovery**: Provide clear paths forward when errors occur

By following these patterns, LLMs can effectively use the PhotoPrism MCP server to accomplish complex photo management tasks while maintaining safety, reliability, and excellent user experience.
