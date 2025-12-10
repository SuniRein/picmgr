use std::env;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = picmgr_server::init_pool(&database_url).await?;

    let app = picmgr_server::create_router(pool);

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:9000".to_string());
    let listener = TcpListener::bind(addr).await?;
    info!(addr = %listener.local_addr().unwrap(), "server started");

    axum::serve(listener, app).await?;

    Ok(())
}
