use super::error::{ApiError, ApiResult};
use crate::{
    api::claims::AdminClaims,
    db::user::{self, User},
};
use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use sqlx::PgPool;
use tracing::{info, instrument};

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

#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_all_users(
    State(pool): State<PgPool>,
    _claims: AdminClaims,
) -> ApiResult<Json<Vec<User>>> {
    let users = user::get_all_users(&pool).await?;
    info!(count = users.len(), "users fetched");
    Ok(Json(users))
}
