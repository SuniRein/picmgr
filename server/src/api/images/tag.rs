use super::super::{
    claims::AccessClaims,
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
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct SetImageTagsRequest {
    pub tags: Vec<String>,
}

/// Set tags for an image
///
/// Allows the owner of the image or an admin to set tags for the specified image.
/// All existing tags will be replaced with the new set of tags provided in the request.
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

    image::set_image_tags(&pool, id, &req.tags).await?;
    info!(tags = ?req.tags, "tags set successfully");

    Ok(StatusCode::NO_CONTENT)
}
