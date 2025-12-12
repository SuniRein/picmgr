use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn create_router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new()
        .routes(routes!(super::get_stats::get_image_count))
        .routes(routes!(super::get_meta::get_image_meta))
        .routes(routes!(super::get_meta::get_image_metas))
        .routes(routes!(super::get_raw::get_image))
        .routes(routes!(super::get_thumbnail::get_thumbnail))
        .routes(routes!(super::upload::upload_raw_image))
        .routes(routes!(
            super::tag::get_image_tags,
            super::tag::set_image_tags,
        ))
}
