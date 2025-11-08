use crate::client::PhotoPrismClient;
use crate::types::*;
use turbomcp::prelude::*;

/// Archive or restore multiple photos in batch
///
/// When to Use:
/// - User wants to hide many photos at once
/// - User wants to restore previously archived photos
/// - Operations like "hide all photos from last year"
///
/// Safety Features:
/// - Max 100 photos per operation
/// - Reversible operation (can restore)
/// - Detailed per-photo status reporting
pub async fn batch_archive_photos(
    client: &PhotoPrismClient,
    ctx: Context,
    photo_uids: Vec<String>,
    restore: Option<bool>,
) -> McpResult<String> {
    if photo_uids.is_empty() {
        return Err(McpError::invalid_request(
            "At least one photo UID must be provided",
        ));
    }

    if photo_uids.len() > 100 {
        return Err(McpError::invalid_request(
            "Maximum 100 photos can be processed at once",
        ));
    }

    let restore = restore.unwrap_or(false);
    let action = if restore { "Restoring" } else { "Archiving" };

    let _ = ctx.info(&format!("{} {} photos...", action, photo_uids.len()))
        .await;

    let mut results = Vec::new();

    for uid in photo_uids {
        let path = if restore {
            format!("/api/v1/photos/{}/restore", uid)
        } else {
            format!("/api/v1/photos/{}/archive", uid)
        };

        match client.post::<(), serde_json::Value>(&path, &()).await {
            Ok(_) => {
                let _ = ctx.info(&format!("Successfully {} photo: {}", action.to_lowercase(), uid))
                    .await;
                results.push(BatchItemResult::success(uid, None));
            }
            Err(e) => {
                let error_msg = format!("Failed to {} photo: {}", action.to_lowercase(), e);
                let _ = ctx.error(&format!("Error for {}: {}", uid, error_msg))
                    .await;
                results.push(BatchItemResult::error(uid, None, error_msg));
            }
        }
    }

    let response = BatchOperationResponse::from_results(
        results,
        Some(format!("Batch {} operation completed", action.to_lowercase())),
    );

    let _ = ctx.info(&format!(
        "Batch operation complete: {} succeeded, {} failed",
        response.summary.success_count, response.summary.error_count
    ))
    .await;

    Ok(serde_json::to_string_pretty(&response)?)
}

/// Delete multiple photos in batch
///
/// DANGER: This is a PERMANENT operation!
///
/// When to Use:
/// - User explicitly confirms deletion
/// - User wants to permanently remove photos
/// - Use cases: "delete all blurry photos", "remove duplicates"
///
/// Safety Features:
/// - Max 50 photos per operation (lower limit for safety)
/// - Requires explicit confirmation parameter
/// - Warning logged for each operation
/// - Detailed per-photo status reporting
pub async fn batch_delete_photos(
    client: &PhotoPrismClient,
    ctx: Context,
    photo_uids: Vec<String>,
    confirm: bool,
    permanent: Option<bool>,
) -> McpResult<String> {
    if !confirm {
        return Err(McpError::invalid_request(
            "Deletion not confirmed. Set confirm=true to proceed. WARNING: This action is permanent!",
        ));
    }

    if photo_uids.is_empty() {
        return Err(McpError::invalid_request(
            "At least one photo UID must be provided",
        ));
    }

    if photo_uids.len() > 50 {
        return Err(McpError::invalid_request(
            "Maximum 50 photos can be deleted at once for safety",
        ));
    }

    let permanent = permanent.unwrap_or(false);

    let _ = ctx.warn(&format!(
        "⚠️  {} deleting {} photos...",
        if permanent { "Permanently" } else { "Soft" },
        photo_uids.len()
    ))
    .await;

    let mut results = Vec::new();

    for uid in photo_uids {
        let path = if permanent {
            format!("/api/v1/photos/{}", uid)
        } else {
            format!("/api/v1/photos/{}/delete", uid)
        };

        match client.delete(&path).await {
            Ok(_) => {
                let _ = ctx.info(&format!("Successfully deleted photo: {}", uid))
                    .await;
                results.push(BatchItemResult::success(uid, None));
            }
            Err(e) => {
                let error_msg = format!("Failed to delete photo: {}", e);
                let _ = ctx.error(&format!("Error for {}: {}", uid, error_msg))
                    .await;
                results.push(BatchItemResult::error(uid, None, error_msg));
            }
        }
    }

    let response = BatchOperationResponse::from_results(
        results,
        Some("Batch delete operation completed".to_string()),
    );

    let _ = ctx.info(&format!(
        "Batch delete complete: {} succeeded, {} failed",
        response.summary.success_count, response.summary.error_count
    ))
    .await;

    Ok(serde_json::to_string_pretty(&response)?)
}

/// Mark multiple photos as favorite or unfavorite in batch
///
/// When to Use:
/// - User wants to favorite many photos at once
/// - User wants to remove favorite status from multiple photos
/// - Operations like "favorite all beach photos"
///
/// Safety Features:
/// - Max 100 photos per operation
/// - Reversible operation
/// - Detailed per-photo status reporting
pub async fn batch_favorite_photos(
    client: &PhotoPrismClient,
    ctx: Context,
    photo_uids: Vec<String>,
    unfavorite: Option<bool>,
) -> McpResult<String> {
    if photo_uids.is_empty() {
        return Err(McpError::invalid_request(
            "At least one photo UID must be provided",
        ));
    }

    if photo_uids.len() > 100 {
        return Err(McpError::invalid_request(
            "Maximum 100 photos can be processed at once",
        ));
    }

    let unfavorite = unfavorite.unwrap_or(false);
    let action = if unfavorite { "Unfavoriting" } else { "Favoriting" };

    let _ = ctx.info(&format!("{} {} photos...", action, photo_uids.len()))
        .await;

    let mut results = Vec::new();

    for uid in photo_uids {
        let path = format!("/api/v1/photos/{}/like", uid);

        let result = if !unfavorite {
            client.post::<(), serde_json::Value>(&path, &()).await
        } else {
            client.delete(&path).await.map(|_| serde_json::Value::Null)
        };

        match result {
            Ok(_) => {
                let _ = ctx.info(&format!("Successfully {} photo: {}", action.to_lowercase(), uid))
                    .await;
                results.push(BatchItemResult::success(uid, None));
            }
            Err(e) => {
                let error_msg = format!("Failed to {} photo: {}", action.to_lowercase(), e);
                let _ = ctx.error(&format!("Error for {}: {}", uid, error_msg))
                    .await;
                results.push(BatchItemResult::error(uid, None, error_msg));
            }
        }
    }

    let response = BatchOperationResponse::from_results(
        results,
        Some(format!("Batch {} operation completed", action.to_lowercase())),
    );

    let _ = ctx.info(&format!(
        "Batch operation complete: {} succeeded, {} failed",
        response.summary.success_count, response.summary.error_count
    ))
    .await;

    Ok(serde_json::to_string_pretty(&response)?)
}

/// Make multiple photos private or public in batch
///
/// When to Use:
/// - User wants to control privacy for many photos
/// - Operations like "make all home photos private"
///
/// Safety Features:
/// - Max 100 photos per operation
/// - Reversible operation
/// - Detailed per-photo status reporting
pub async fn batch_private_photos(
    client: &PhotoPrismClient,
    ctx: Context,
    photo_uids: Vec<String>,
    make_public: Option<bool>,
) -> McpResult<String> {
    if photo_uids.is_empty() {
        return Err(McpError::invalid_request(
            "At least one photo UID must be provided",
        ));
    }

    if photo_uids.len() > 100 {
        return Err(McpError::invalid_request(
            "Maximum 100 photos can be processed at once",
        ));
    }

    let make_public = make_public.unwrap_or(false);
    let action = if make_public { "Making public" } else { "Making private" };

    let _ = ctx.info(&format!("{} {} photos...", action, photo_uids.len()))
        .await;

    let mut results = Vec::new();

    use serde::Serialize;
    #[derive(Serialize)]
    struct PrivacyUpdate {
        #[serde(rename = "Private")]
        private: bool,
    }

    for uid in photo_uids {
        let path = format!("/api/v1/photos/{}", uid);
        let update = PrivacyUpdate { private: !make_public };

        match client.put::<PrivacyUpdate, serde_json::Value>(&path, &update).await {
            Ok(_) => {
                let _ = ctx.info(&format!("Successfully {} photo: {}", action.to_lowercase(), uid))
                    .await;
                results.push(BatchItemResult::success(uid, None));
            }
            Err(e) => {
                let error_msg = format!("Failed to {} photo: {}", action.to_lowercase(), e);
                let _ = ctx.error(&format!("Error for {}: {}", uid, error_msg))
                    .await;
                results.push(BatchItemResult::error(uid, None, error_msg));
            }
        }
    }

    let response = BatchOperationResponse::from_results(
        results,
        Some(format!("{} operation completed", action)),
    );

    let _ = ctx.info(&format!(
        "Batch operation complete: {} succeeded, {} failed",
        response.summary.success_count, response.summary.error_count
    ))
    .await;

    Ok(serde_json::to_string_pretty(&response)?)
}

/// Update title, description, or tags for multiple photos in batch
///
/// When to Use:
/// - User wants to apply same metadata to many photos
/// - Operations like "add 'vacation' tag to all Hawaii photos"
///
/// Safety Features:
/// - Max 50 photos per operation (metadata updates are heavier)
/// - Requires at least one update field
/// - Detailed per-photo status reporting
pub async fn batch_update_photos(
    client: &PhotoPrismClient,
    ctx: Context,
    photo_uids: Vec<String>,
    title: Option<String>,
    description: Option<String>,
    add_tags: Option<Vec<String>>,
) -> McpResult<String> {
    if photo_uids.is_empty() {
        return Err(McpError::invalid_request(
            "At least one photo UID must be provided",
        ));
    }

    if photo_uids.len() > 50 {
        return Err(McpError::invalid_request(
            "Maximum 50 photos can be updated at once",
        ));
    }

    if title.is_none() && description.is_none() && add_tags.is_none() {
        return Err(McpError::invalid_request(
            "At least one update parameter (title, description, or add_tags) must be provided",
        ));
    }

    let _ = ctx.info(&format!("Updating {} photos...", photo_uids.len()))
        .await;

    let mut results = Vec::new();

    use serde::Serialize;
    #[derive(Serialize)]
    struct PhotoUpdate {
        #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    }

    for uid in photo_uids {
        let path = format!("/api/v1/photos/{}", uid);
        let update = PhotoUpdate {
            title: title.clone(),
            description: description.clone(),
        };

        match client.put::<PhotoUpdate, serde_json::Value>(&path, &update).await {
            Ok(_) => {
                let _ = ctx.info(&format!("Successfully updated photo: {}", uid))
                    .await;
                results.push(BatchItemResult::success(uid, None));
            }
            Err(e) => {
                let error_msg = format!("Failed to update photo: {}", e);
                let _ = ctx.error(&format!("Error for {}: {}", uid, error_msg))
                    .await;
                results.push(BatchItemResult::error(uid, None, error_msg));
            }
        }
    }

    let response = BatchOperationResponse::from_results(
        results,
        Some("Batch update operation completed".to_string()),
    );

    let _ = ctx.info(&format!(
        "Batch update complete: {} succeeded, {} failed",
        response.summary.success_count, response.summary.error_count
    ))
    .await;

    Ok(serde_json::to_string_pretty(&response)?)
}
