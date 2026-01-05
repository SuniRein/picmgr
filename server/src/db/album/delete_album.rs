use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(pool))]
pub async fn delete_album(pool: &PgPool, id: i32) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM album WHERE id = $1", id)
        .execute(pool)
        .await
        .inspect_err(|e| error!(error=?e, "delete album failed"))?;
    Ok(())
}
