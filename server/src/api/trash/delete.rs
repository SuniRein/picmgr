use super::super::{
    claims::UserClaims,
    doc::TRASH_TAG,
    error::{ApiError, ApiResult},
    images::utils::check_user_permission,
    stats::CountResponse,
};
use crate::db::image::{self, DeleteImageResult};
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use tracing::{info, instrument};

/// Permanently deletes a trashed image.
///
/// The image will be permanently removed if it exists in the trash.
/// Only the owner of the image can perform this action.
#[utoipa::path(
    delete,
    tag = TRASH_TAG,
    path = "/trash/images/{id}",
    security(("userAuth" = [])),
    responses(
        (status = NO_CONTENT, description = "image deleted successfully"),
        (status = NOT_FOUND, description = "image not found"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn delete_trashed_image(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(image_id): Path<i32>,
) -> ApiResult<impl IntoResponse> {
    check_user_permission(&pool, image_id, claims.user_id()).await?;

    match image::delete_trashed_image(&pool, image_id).await? {
        DeleteImageResult::Deleted => {
            info!("image restored successfully");
            Ok(StatusCode::NO_CONTENT)
        }
        DeleteImageResult::NotFound => {
            info!("image exists but not in trash");
            Err(ApiError::NotFound)
        }
    }
}

/// Permanently deletes all trashed images.
///
/// The images will be permanently removed if they exist in the trash.
#[utoipa::path(
    delete,
    tag = TRASH_TAG,
    path = "/trash/images",
    security(("userAuth" = [])),
    responses(
        (status = OK, description = "all trashed images deleted successfully", body = CountResponse),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn delete_all_user_trashed_images(
    State(pool): State<PgPool>,
    claims: UserClaims,
) -> ApiResult<Json<CountResponse>> {
    let count = image::delete_all_user_trashed_images(&pool, claims.user_id()).await?;
    info!(count, "trashed image deleted");
    Ok(Json(CountResponse {
        count: count as i64,
    }))
}
