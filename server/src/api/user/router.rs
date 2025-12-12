use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn create_router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new().routes(routes!(super::get_current::get_current_user))
}
