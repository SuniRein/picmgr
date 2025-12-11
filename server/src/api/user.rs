use super::error::{ApiError, ApiResult};
use crate::{
    api::claims::UserClaims,
    db::user::{self, User},
};
use axum::{Json, debug_handler, extract::State};
use sqlx::PgPool;
use tracing::{info, instrument, warn};

#[debug_handler]
#[instrument(skip(pool, claims))]
pub async fn get_current_user(
    State(pool): State<PgPool>,
    claims: UserClaims,
) -> ApiResult<Json<User>> {
    let user = user::get_user_by_id(&pool, claims.user_id()).await?;
    match user {
        Some(user) => {
            info!("current user fetched");
            Ok(Json(user))
        }
        None => {
            warn!("invalid token, user not found");
            Err(ApiError::InvalidToken)
        }
    }
}
