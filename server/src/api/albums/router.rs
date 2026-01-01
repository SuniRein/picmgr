use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn create_router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new()
        .routes(routes!(super::create_album::create_album))
        .routes(routes!(super::get_album::get_albums))
        .routes(routes!(super::get_album::get_album_by_id))
        .routes(routes!(super::get_stats::get_album_count))
}
