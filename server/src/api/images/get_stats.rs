use super::super::{claims::AccessClaims, doc::IMAGES_TAG, error::ApiResult, stats::CountResponse};
use crate::db::image;
use axum::{Json, debug_handler, extract::State};
use sqlx::PgPool;
use tracing::{info, instrument};

/// Get total image count
///
/// Retrieves the total number of images that owned by the authenticated user.
/// Returns the total count of all images for admin.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/count",
    security(
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = CountResponse),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_image_count(
    State(pool): State<PgPool>,
    claims: AccessClaims,
) -> ApiResult<Json<CountResponse>> {
    Ok(match claims {
        AccessClaims::Admin => image::get_total_image_count(&pool).await,
        AccessClaims::User(user_id) => image::get_user_image_count(&pool, user_id).await,
    }
    .map(|count| {
        info!(count, "image count fetched");
        Json(CountResponse { count })
    })?)
}
