use crate::api::user;
use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/users/{id}", get(user::get_user))
        .route("/api/users", get(user::get_all_users))
        .with_state(pool)
}
