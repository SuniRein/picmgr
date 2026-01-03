use super::{
    super::{
        claims::UserClaims,
        doc::IMAGES_TAG,
        error::{ApiError, ApiResult},
        pagination::{PaginatedResponse, PaginationQuery},
        stats::CountResponse,
    },
    get_meta::ImageMetaResponse,
};
use crate::db::image;
use axum::{
    Json, debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{info, instrument};

/// Get metadata for trashed images
///
/// Retrieves metadata for images that have been moved to trash by the authenticated user.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/trash",
    security(("userAuth" = [])),
    params(PaginationQuery),
    responses(
        (status = OK, description = "success response", body = PaginatedResponse<ImageMetaResponse>),
        (status = BAD_REQUEST, description = "invalid pagination parameters"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_trashed_image_metas(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Query(pagination): Query<PaginationQuery>,
) -> ApiResult<Json<PaginatedResponse<ImageMetaResponse>>> {
    let pagination = pagination.check()?;

    let metas =
        image::get_user_trashed_image_metas(&pool, claims.user_id(), pagination.into()).await?;
    info!(count = metas.len(), "trashed images fetched");

    let now = Utc::now();
    Ok(Json(PaginatedResponse::new(
        metas
            .into_iter()
            .map(|meta| ImageMetaResponse::generate_from(meta, now))
            .collect(),
        pagination,
    )))
}

/// Get trashed image count
///
/// Retrieves the total count of images owned by the authenticated user
/// that have been moved to trash.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/trash/count",
    security(("userAuth" = [])),
    responses(
        (status = OK, description = "success response", body = CountResponse),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_trashed_image_count(
    State(pool): State<PgPool>,
    claims: UserClaims,
) -> ApiResult<Json<CountResponse>> {
    Ok(image::get_user_trashed_image_count(&pool, claims.user_id())
        .await
        .map(|count| {
            info!(count, "trashed image count fetched");
            Json(CountResponse { count })
        })?)
}

/// Trash an image
///
/// Moves the specified image to trash for the authenticated user.
#[utoipa::path(
    post,
    tag = IMAGES_TAG,
    path = "/images/{id}/trash",
    security(("userAuth" = [])),
    responses(
        (status = NO_CONTENT, description = "image trashed successfully"),
        (status = NOT_FOUND, description = "image not found"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn trash_image(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(image_id): Path<i32>,
) -> ApiResult<impl IntoResponse> {
    check_permission(&pool, image_id, claims.user_id()).await?;
    image::trash_image(&pool, image_id).await?;
    info!("image trashed successfully");
    Ok(StatusCode::NO_CONTENT)
}

/// Restore a trashed image
///
/// Restores the specified image from trash for the authenticated user.
#[utoipa::path(
    post,
    tag = IMAGES_TAG,
    path = "/images/{id}/restore",
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
    check_permission(&pool, image_id, claims.user_id()).await?;
    image::restore_trashed_image(&pool, image_id).await?;
    info!("image restored successfully");
    Ok(StatusCode::NO_CONTENT)
}

async fn check_permission(pool: &PgPool, image_id: i32, user_id: i32) -> ApiResult<()> {
    let permission = image::get_image_permission(pool, image_id)
        .await?
        .ok_or_else(|| {
            info!("image not found");
            ApiError::NotFound
        })?;
    if permission.owner_id != Some(user_id) {
        info!("permission denied");
        return Err(ApiError::PermissionDenied);
    }
    Ok(())
}
