use super::super::{
    claims::UserClaims,
    doc::TRASH_TAG,
    error::ApiResult,
    images::sign::SignedQuery,
    pagination::{PaginatedResponse, PaginationQuery},
    stats::CountResponse,
};
use crate::db::image::{self, TrashedImageMeta};
use axum::{
    Json, debug_handler,
    extract::{Query, State},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use tracing::{info, instrument};

#[derive(Serialize, utoipa::ToSchema)]
pub struct TrashedImageMetaResponse {
    meta: TrashedImageMeta,
    signature: SignedQuery,
}

impl TrashedImageMetaResponse {
    pub fn generate_from(meta: TrashedImageMeta, now: DateTime<Utc>) -> Self {
        let id = meta.id;
        let signature = SignedQuery::generate(id, now);
        Self { meta, signature }
    }
}

/// Get metadata for trashed images
///
/// Retrieves metadata for images that have been moved to trash by the authenticated user.
#[utoipa::path(
    get,
    tag = TRASH_TAG,
    path = "/trash/images",
    security(("userAuth" = [])),
    params(PaginationQuery),
    responses(
        (status = OK, description = "success response", body = PaginatedResponse<TrashedImageMetaResponse>),
        (status = BAD_REQUEST, description = "invalid pagination parameters"),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_trashed_image_metas(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Query(pagination): Query<PaginationQuery>,
) -> ApiResult<Json<PaginatedResponse<TrashedImageMetaResponse>>> {
    let pagination = pagination.check()?;

    let metas =
        image::get_user_trashed_image_metas(&pool, claims.user_id(), pagination.into()).await?;
    info!(count = metas.len(), "trashed images fetched");

    let now = Utc::now();
    Ok(Json(PaginatedResponse::new(
        metas
            .into_iter()
            .map(|meta| TrashedImageMetaResponse::generate_from(meta, now))
            .collect(),
        pagination,
    )))
}

/// Get trashed image count
///
/// Retrieves the total count of images owned by the authenticated user
/// that have been moved to trash.
#[utoipa::path(
    get,
    tag = TRASH_TAG,
    path = "/trash/images/count",
    security(("userAuth" = [])),
    responses(
        (status = OK, description = "success response", body = CountResponse),
        (status = UNAUTHORIZED, description = "invalid or missing token"),
    ),
)]
#[debug_handler]
#[instrument(skip(pool))]
pub async fn get_trashed_image_count(
    State(pool): State<PgPool>,
    claims: UserClaims,
) -> ApiResult<Json<CountResponse>> {
    Ok(image::get_user_trashed_image_count(&pool, claims.user_id())
        .await
        .map(|count| {
            info!(count, "trashed image count fetched");
            Json(CountResponse { count })
        })?)
}
