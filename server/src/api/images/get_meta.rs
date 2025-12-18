use super::{
    super::{
        claims::{AccessClaims, AnyClaims},
        doc::IMAGES_TAG,
        error::ApiResult,
        pagination::{PaginatedResponse, PaginationQuery},
    },
    sign::SignedQuery,
    utils::get_image_info,
};
use crate::{
    api::pagination::PaginationChecked,
    db::image::{self, ImageMeta},
};
use axum::{
    Json, debug_handler,
    extract::{Path, Query, State},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Serialize, utoipa::ToSchema)]
pub struct ImageMetaResponse {
    meta: ImageMeta,
    url: String,
}

impl ImageMetaResponse {
    fn generate_from(meta: ImageMeta, now: DateTime<Utc>) -> Self {
        let id = meta.id;
        let query = SignedQuery::generate(id, now).to_query_string();
        let url = format!("/images/{id}/raw/signed?{query}");
        Self { meta, url }
    }
}

/// Get metadata for all images
///
/// Retrieves metadata for all images if the requester has admin access.
/// For regular users, retrieves metadata only for images they own.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images",
    security(
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    params(PaginationQuery),
    responses(
        (status = OK, description = "success response", body = PaginatedResponse<ImageMetaResponse>),
        (status = BAD_REQUEST, description = "invalid pagination parameters"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_image_metas(
    State(pool): State<PgPool>,
    claims: AccessClaims,
    Query(pagination): Query<PaginationQuery>,
) -> ApiResult<Json<PaginatedResponse<ImageMetaResponse>>> {
    let pagination = pagination.check()?;
    match claims {
        AccessClaims::Admin => get_all_image_metas(&pool, pagination).await,
        AccessClaims::User(user_id) => get_user_image_metas(&pool, user_id, pagination).await,
    }
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
    })
}

async fn get_all_image_metas(
    pool: &PgPool,
    pagination: PaginationChecked,
) -> ApiResult<Vec<ImageMeta>> {
    let images = image::get_all_image_metas(pool, pagination.into()).await?;
    Ok(images.into_iter().collect())
}

async fn get_user_image_metas(
    pool: &PgPool,
    user_id: i32,
    pagination: PaginationChecked,
) -> ApiResult<Vec<ImageMeta>> {
    let images = image::get_image_metas_by_owner(pool, user_id, pagination.into()).await?;
    Ok(images.into_iter().collect())
}

/// Get image metadata by ID
///
/// Retrieves metadata for a specific image if the requester has the necessary permissions.
#[utoipa::path(
    get,
    tag = IMAGES_TAG,
    path = "/images/{id}",
    security(
        (),
        ("userAuth" = []),
        ("adminAuth" = [])
    ),
    responses(
        (status = OK, description = "success response", body = ImageMetaResponse),
        (status = FORBIDDEN, description = "permission denied"),
        (status = NOT_FOUND, description = "image not found"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_image_meta(
    State(pool): State<PgPool>,
    claims: AnyClaims,
    Path(image_id): Path<i32>,
) -> ApiResult<Json<ImageMetaResponse>> {
    get_image_info(image::get_image_meta_by_id, &pool, claims, image_id)
        .await
        .map(|meta| {
            info!("image meta fetched");
            Json(ImageMetaResponse::generate_from(meta, Utc::now()))
        })
}
