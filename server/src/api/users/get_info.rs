use super::super::{
    claims::AdminClaims,
    doc::USERS_TAG,
    error::{ApiError, ApiResult},
    pagination::{PaginatedResponse, PaginationQuery},
};
use crate::db::user::{self, User};
use axum::{
    Json, debug_handler,
    extract::{Path, Query, State},
};
use sqlx::PgPool;
use tracing::{info, instrument};

/// Get user information by user ID
///
/// Returns the user details for the specified user ID.
#[utoipa::path(
    get,
    tag = USERS_TAG,
    path = "/users/{id}",
    security(("adminAuth" = [])),
    responses(
        (status = OK, description = "success response", body = User),
        (status = NOT_FOUND, description = "user not found"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = FORBIDDEN, description = "admin privileges required"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_user(
    State(pool): State<PgPool>,
    _claims: AdminClaims,
    Path(user_id): Path<i32>,
) -> ApiResult<Json<User>> {
    let user = user::get_user_by_id(&pool, user_id).await?;
    match user {
        Some(user) => {
            info!("user fetched");
            Ok(Json(user))
        }
        None => {
            info!("user not found");
            Err(ApiError::NotFound)
        }
    }
}

/// Get information of all users
///
/// Return a list of all user details.
#[utoipa::path(
    get,
    tag = USERS_TAG,
    path = "/users",
    security(("adminAuth" = [])),
    params(PaginationQuery),
    responses(
        (status = OK, description = "success response", body = PaginatedResponse<User>),
        (status = BAD_REQUEST, description = "invalid pagination parameters"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = FORBIDDEN, description = "admin privileges required"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_all_users(
    State(pool): State<PgPool>,
    _claims: AdminClaims,
    Query(pagination): Query<PaginationQuery>,
) -> ApiResult<Json<PaginatedResponse<User>>> {
    let pagination = pagination.check()?;
    Ok(user::get_all_users(&pool, pagination.into())
        .await
        .map(|users| {
            info!(count = users.len(), "users fetched");
            Json(PaginatedResponse::new(users, pagination))
        })?)
}
