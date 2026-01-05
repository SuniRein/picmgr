use super::{
    super::{claims::UserClaims, doc::ALBUMS_TAG, error::ApiResult},
    utils::check_permission,
};
use crate::db::album;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use tracing::{info, instrument};

/// Delete an album
///
/// Only the owner of the album can do this.
/// It just remove the album records, while images in the album stay as is.
#[utoipa::path(
    delete,
    tag = ALBUMS_TAG,
    path = "/albums/{id}",
    security(("userAuth" = [])),
    responses(
        (status = NO_CONTENT, description = "album deleted successfully"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = CONFLICT, description = "album name already exists"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn delete_album(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(id): Path<i32>,
) -> ApiResult<impl IntoResponse> {
    check_permission(&pool, id, claims.user_id()).await?;
    album::delete_album(&pool, id).await?;
    info!("album deleted successfully");
    Ok(StatusCode::NO_CONTENT)
}
