use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn create_router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new()
        .routes(routes!(super::get_images::get_trashed_image_metas))
        .routes(routes!(super::get_images::get_trashed_image_count))
        .routes(routes!(super::restore::restore_trashed_image))
}
