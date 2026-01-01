use super::{
    super::{
        claims::UserClaims,
        doc::ALBUMS_TAG,
        error::ApiResult,
        images::ImageMetaResponse,
        pagination::{PaginatedResponse, PaginationQuery},
    },
    utils::check_permission,
};
use crate::db::album;
use axum::{
    Json, debug_handler,
    extract::{Path, Query, State},
};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{info, instrument};

/// Get image metas in an album
///
/// Retrieves metadata for all images in the specified album.
#[utoipa::path(
    get,
    tag = ALBUMS_TAG,
    path = "/albums/{id}/images",
    security(
        ("userAuth" = []),
    ),
    params(PaginationQuery),
    responses(
        (status = OK, description = "success response", body = PaginatedResponse<ImageMetaResponse>),
        (status = BAD_REQUEST, description = "invalid pagination parameters"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = NOT_FOUND, description = "album not found"),
        (status = FORBIDDEN, description = "permission denied"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_images_in_album(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(id): Path<i32>,
    Query(pagination): Query<PaginationQuery>,
) -> ApiResult<Json<PaginatedResponse<ImageMetaResponse>>> {
    let pagination = pagination.check()?;

    check_permission(&pool, id, claims.user_id()).await?;

    Ok(album::get_images_in_album(&pool, id, pagination.into())
        .await
        .map(|metas| {
            info!(count = metas.len(), "images fetched");

            let now = Utc::now();
            Json(PaginatedResponse::new(
                metas
                    .into_iter()
                    .map(|meta| ImageMetaResponse::generate_from(meta, now))
                    .collect(),
                pagination,
            ))
        })?)
}
