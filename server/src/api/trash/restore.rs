use super::super::{
    claims::UserClaims, doc::TRASH_TAG, error::ApiResult, images::utils::check_user_permission,
};
use crate::db::image;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use tracing::{info, instrument};

/// Restore a trashed image
///
/// Restores the specified image from trash for the authenticated user.
#[utoipa::path(
    post,
    tag = TRASH_TAG,
    path = "/trash/images/{id}/restore",
    security(("userAuth" = [])),
    responses(
        (status = NO_CONTENT, description = "image restored successfully"),
        (status = NOT_FOUND, description = "image not found"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn restore_trashed_image(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(image_id): Path<i32>,
) -> ApiResult<impl IntoResponse> {
    check_user_permission(&pool, image_id, claims.user_id()).await?;
    image::restore_trashed_image(&pool, image_id).await?;
    info!("image restored successfully");
    Ok(StatusCode::NO_CONTENT)
}
