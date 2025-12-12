use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn create_router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new()
        .routes(routes!(super::get_stats::get_user_count))
        .routes(routes!(super::get_info::get_user))
        .routes(routes!(super::get_info::get_all_users))
}
