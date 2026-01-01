use super::{
    super::{claims::UserClaims, doc::ALBUMS_TAG, error::ApiResult, stats::CountResponse},
    utils::check_permission,
};
use crate::db::album;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use sqlx::PgPool;
use tracing::{info, instrument};

/// Get total album count
///
/// Retrieves the total number of albums that owned by the authenticated user.
#[utoipa::path(
    get,
    tag = ALBUMS_TAG,
    path = "/albums/count",
    security(
        ("userAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = CountResponse),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_album_count(
    State(pool): State<PgPool>,
    claims: UserClaims,
) -> ApiResult<Json<CountResponse>> {
    Ok(album::get_user_album_count(&pool, claims.user_id())
        .await
        .map(|count| {
            info!(count, "album count fetched");
            Json(CountResponse { count })
        })?)
}

/// Get total image count in an album
///
/// Retrieves the total number of images in the specified album owned by the authenticated user.
#[utoipa::path(
    get,
    tag = ALBUMS_TAG,
    path = "/albums/{id}/images/count",
    security(
        ("userAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = CountResponse),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = NOT_FOUND, description = "album not found"),
        (status = FORBIDDEN, description = "permission denied"),
    )
)]
pub async fn get_image_count_in_album(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(id): Path<i32>,
) -> ApiResult<Json<CountResponse>> {
    check_permission(&pool, id, claims.user_id()).await?;
    Ok(album::get_image_count_in_album(&pool, id)
        .await
        .map(|count| {
            info!(count, "image count fetched");
            Json(CountResponse { count })
        })?)
}
