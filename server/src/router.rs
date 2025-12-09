use crate::api::{auth, user, users};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    let api = Router::new()
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/login/admin", post(auth::login_as_admin))
        .route("/user/me", get(user::get_current_user))
        .route("/users/{id}", get(users::get_user))
        .route("/users", get(users::get_all_users));

    Router::new().nest("/api", api).with_state(pool)
}
