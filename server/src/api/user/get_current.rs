use super::super::{
    claims::UserClaims,
    doc::USER_TAG,
    error::{ApiError, ApiResult},
};
use crate::db::user::{self, User};
use axum::{Json, debug_handler, extract::State};
use sqlx::PgPool;
use tracing::{info, instrument, warn};

/// Get the currently authenticated user's information
///
/// Returns the user details of the authenticated user based on the provided JWT token.
#[utoipa::path(
    get,
    tag = USER_TAG,
    path = "/user/me",
    security(("userAuth" = [])),
    responses(
        (status = OK, description = "success response", body = User),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
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
