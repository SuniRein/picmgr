use super::super::{claims::AdminClaims, doc::USERS_TAG, error::ApiResult, stats::CountResponse};
use crate::db::user;
use axum::{Json, debug_handler, extract::State};
use sqlx::PgPool;
use tracing::{info, instrument};

/// Get total user count
///
/// Retrieves the total number of users in the system. Accessible only by admin users.
#[utoipa::path(
    get,
    tag = USERS_TAG,
    path = "/users/count",
    security(
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = CountResponse),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = FORBIDDEN, description = "admin privileges required"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_user_count(
    State(pool): State<PgPool>,
    claims: AdminClaims,
) -> ApiResult<Json<CountResponse>> {
    Ok(user::get_total_user_count(&pool).await.map(|count| {
        info!(count, "user count fetched successfully");
        Json(CountResponse { count })
    })?)
}
