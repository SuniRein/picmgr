use super::ApiError;
use crate::db::user::{self, User};
use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use sqlx::PgPool;

type ApiResult<T> = Result<Json<T>, ApiError>;

#[debug_handler]
pub async fn get_user(Path(user_id): Path<i32>, State(pool): State<PgPool>) -> ApiResult<User> {
    let user = user::get_user_by_id(&pool, user_id).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(ApiError::NotFound),
    }
}

#[debug_handler]
pub async fn get_all_users(State(pool): State<PgPool>) -> ApiResult<Vec<User>> {
    let users = user::get_all_users(&pool).await?;
    Ok(Json(users))
}
