use super::{
    super::{
        claims::UserClaims,
        doc::ALBUMS_TAG,
        error::{ApiError, ApiResult},
    },
    utils::check_permission,
};
use crate::db::album;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use tracing::{info, instrument};

/// Remove an image from an album
///
/// Removes an existing image from a specified album for the authenticated user.
#[utoipa::path(
    delete,
    tag = ALBUMS_TAG,
    path = "/albums/{album_id}/images/{image_id}",
    security(("userAuth" = [])),
    responses(
        (status = NO_CONTENT, description = "image removed from album successfully"),
        (status = NOT_FOUND, description = "album or image not found"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn remove_image_from_album(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path((album_id, image_id)): Path<(i32, i32)>,
) -> ApiResult<impl IntoResponse> {
    check_permission(&pool, album_id, claims.user_id()).await?;

    let removed = album::remove_image_from_album(&pool, album_id, image_id).await?;
    if removed {
        info!("image removed from album successfully");
        Ok(StatusCode::NO_CONTENT)
    } else {
        info!("image not found");
        Err(ApiError::NotFound)
    }
}
