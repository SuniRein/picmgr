use super::super::{
    claims::UserClaims,
    doc::ALBUMS_TAG,
    error::{ApiError, ApiResult},
    pagination::{PaginatedResponse, PaginationQuery},
};
use crate::db::album::{self, Album};
use axum::{
    Json, debug_handler,
    extract::{Path, Query, State},
};
use sqlx::PgPool;
use tracing::{info, instrument};

/// Get all album metas owned by current user
///
/// Retrieves a list of album metas owned by the user.
#[utoipa::path(
    get,
    tag = ALBUMS_TAG,
    path = "/albums",
    security(
        ("userAuth" = []),
    ),
    params(PaginationQuery),
    responses(
        (status = OK, description = "success response", body = PaginatedResponse<Album>),
        (status = BAD_REQUEST, description = "invalid pagination parameters"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_albums(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Query(pagination): Query<PaginationQuery>,
) -> ApiResult<Json<PaginatedResponse<Album>>> {
    let pagination = pagination.check()?;
    let albums = album::get_albums_by_owner(&pool, claims.user_id(), pagination.into()).await?;
    info!(count = albums.len(), "albums fetched");
    Ok(Json(PaginatedResponse::new(albums, pagination)))
}

/// Get album meta by id
///
/// Retrieves album meta by its ID.
#[utoipa::path(
    get,
    tag = ALBUMS_TAG,
    path = "/albums/{id}",
    security(
        ("userAuth" = []),
    ),
    params(
        ("id" = i32, Path, description = "The ID of the album to retrieve"),
    ),
    responses(
        (status = OK, description = "success response", body = Album),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = NOT_FOUND, description = "image not found"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_album_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    claims: UserClaims,
) -> ApiResult<Json<Album>> {
    let album = album::get_album_by_id(&pool, id).await?;
    match album {
        Some(album) if album.owner_id == claims.user_id() => {
            info!("album fetched");
            Ok(Json(album))
        }
        Some(_) => {
            info!("permission denied to access album");
            Err(ApiError::PermissionDenied)
        }
        None => {
            info!("album not found");
            Err(ApiError::NotFound)
        }
    }
}
