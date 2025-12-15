use super::super::{
    claims::{AccessClaims, AnyClaims},
    doc::IMAGES_TAG,
    error::{ApiError, ApiResult},
};
use crate::db::image;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Serialize, utoipa::ToSchema)]
pub struct TagList {
    pub tags: Vec<String>,
}

/// Get tags for an image
///
/// Fetches the list of tags associated with the specified image.
/// Public images' tags can be accessed by any authenticated user.
/// Only the owner of the image or an admin can access the tags of a private image.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/{id}/tags",
    security(
        (),
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = TagList),
        (status = NOT_FOUND, description = "image not found"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = FORBIDDEN, description = "permission denied"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_image_tags(
    State(pool): State<PgPool>,
    claims: AnyClaims,
    Path(id): Path<i32>,
) -> ApiResult<impl IntoResponse> {
    let permission = image::get_image_permission(&pool, id)
        .await?
        .ok_or_else(|| {
            info!("image not found");
            ApiError::NotFound
        })?;

    let accessible = matches!(claims, AnyClaims::Admin)
        || permission.is_public
        || matches!(claims, AnyClaims::User(user_id) if permission.owner_id == Some(user_id));
    if !accessible {
        info!("permission denied");
        return Err(ApiError::PermissionDenied);
    }

    Ok(image::get_image_tags(&pool, id).await.map(|tags| {
        info!("image tags fetched");
        Json(TagList { tags })
    })?)
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct SetImageTagsRequest {
    pub tags: Vec<String>,
}

/// Set tags for an image
///
/// Allows the owner of the image or an admin to set tags for the specified image.
/// All existing tags will be replaced with the new set of tags provided in the request.
///
/// Tags are normalized by trimming whitespace and deduplicating.
/// Empty tags are not allowed.
#[utoipa::path(
    put,
    tag = IMAGES_TAG,
    path = "/images/{id}/tags",
    security(
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = NO_CONTENT, description = "tags set successfully"),
        (status = NOT_FOUND, description = "image not found"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = UNPROCESSABLE_ENTITY, description = "invalid tag list: empty tag"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool, req))]
pub async fn set_image_tags(
    State(pool): State<PgPool>,
    claims: AccessClaims,
    Path(id): Path<i32>,
    Json(req): Json<SetImageTagsRequest>,
) -> ApiResult<impl IntoResponse> {
    let owner_id = image::get_image_permission(&pool, id)
        .await?
        .ok_or_else(|| {
            info!("image not found");
            ApiError::NotFound
        })?
        .owner_id;

    if let AccessClaims::User(user_id) = claims
        && owner_id != Some(user_id)
    {
        info!(owner_id, "not the owner of the image");
        return Err(ApiError::PermissionDenied);
    }

    // Normalize and validate tags: trim, dedupe; reject empties
    let mut normalized: Vec<String> = Vec::with_capacity(req.tags.len());
    for t in &req.tags {
        let s = t.trim();
        if s.is_empty() {
            return Err(ApiError::EmptyField("tag".to_string()));
        }
        normalized.push(s.to_string());
    }
    normalized.sort_unstable();
    normalized.dedup();

    image::set_image_tags(&pool, id, &normalized).await?;
    info!(tags = ?normalized, "tags set");

    Ok(StatusCode::NO_CONTENT)
}
