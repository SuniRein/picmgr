use super::{
    super::{
        claims::UserClaims,
        doc::ALBUMS_TAG,
        error::{ApiError, ApiResult},
    },
    utils::check_permission,
};
use crate::db::album::{self, Album, UpdateAlbumInput, UpdateAlbumResult};
use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct UpdateAlbumPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

/// Update an existing album
///
/// Updates the metadata of an existing album for the authenticated user.
#[utoipa::path(
    patch,
    tag = ALBUMS_TAG,
    path = "/albums/{id}",
    security(("userAuth" = [])),
    responses(
        (status = OK, description = "album updated successfully", body = Album),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
        (status = FORBIDDEN, description = "permission denied"),
        (status = NOT_FOUND, description = "album not found"),
        (status = CONFLICT, description = "album name already exists"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool, payload))]
pub async fn update_album(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAlbumPayload>,
) -> ApiResult<Json<Album>> {
    check_permission(&pool, id, claims.user_id()).await?;

    let update_input = UpdateAlbumInput {
        album_id: id,
        name: payload.name.as_deref(),
        description: payload.description.as_deref(),
        is_public: payload.is_public,
    };

    match album::update_album(&pool, update_input).await? {
        UpdateAlbumResult::Updated(album) => {
            info!("album updated successfully");
            Ok(Json(album))
        }
        UpdateAlbumResult::DuplicateName => {
            info!("album name conflict");
            Err(ApiError::AlbumNameConflict)
        }
    }
}
