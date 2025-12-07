use axum::{Router, routing::get};
use sqlx::postgres::PgPool;
use std::env;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    info!("Connected to the database");

    sqlx::migrate!().run(&pool).await?;

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:9000".to_string());
    let listener = TcpListener::bind(addr).await?;
    info!("Server running on {}", listener.local_addr().unwrap());

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::serve(listener, app).await?;

    Ok(())
}
