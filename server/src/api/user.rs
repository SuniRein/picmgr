use super::error::{ApiError, ApiResult};
use crate::{
    api::jwt::UserClaims,
    db::user::{self, User},
};
use axum::{Json, debug_handler, extract::State};
use sqlx::PgPool;

#[debug_handler]
pub async fn get_current_user(
    claims: UserClaims,
    State(pool): State<PgPool>,
) -> ApiResult<Json<User>> {
    let user = user::get_user_by_id(&pool, claims.user_id()).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(ApiError::InvalidToken),
    }
}
