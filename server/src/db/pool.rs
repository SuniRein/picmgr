use sqlx::PgPool;
use tracing::info;

#[tracing::instrument(skip(database_url))]
pub async fn init_pool(database_url: &str) -> sqlx::Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;
    sqlx::migrate!().run(&pool).await?;
    info!("Connected to the database and ran migrations");
    Ok(pool)
}
