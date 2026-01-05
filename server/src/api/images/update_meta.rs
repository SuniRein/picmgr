use super::{
    super::{claims::UserClaims, doc::IMAGES_TAG, error::ApiResult},
    utils::check_user_permission,
};
use crate::db::image;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateImagePayload {
    pub is_public: Option<bool>,
}

/// Update image metadata.
///
/// Updates the metadata of an image for the authenticated user.
#[utoipa::path(
    patch,
    tag = IMAGES_TAG,
    path = "/images/{id}",
    security(("userAuth" = [])),
    responses(
        (status = NO_CONTENT, description = "image updated successfully"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = NOT_FOUND, description = "image not found"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool, payload))]
pub async fn update_image_meta(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateImagePayload>,
) -> ApiResult<impl IntoResponse> {
    check_user_permission(&pool, id, claims.user_id()).await?;

    if let Some(is_public) = payload.is_public {
        image::set_visibility(&pool, id, is_public).await?;
    }
    info!("image meta updated successfully");
    Ok(StatusCode::NO_CONTENT)
}
