use sqlx::PgPool;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn create_router() -> OpenApiRouter<PgPool> {
    OpenApiRouter::new()
        .routes(routes!(super::login::login))
        .routes(routes!(super::login::login_as_admin))
        .routes(routes!(super::refresh::refresh_token))
        .routes(routes!(super::register::register))
}
