use super::super::{claims::UserClaims, doc::ALBUMS_TAG, error::ApiResult, stats::CountResponse};
use crate::db::album;
use axum::{Json, debug_handler, extract::State};
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
