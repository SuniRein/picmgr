use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(pool))]
pub async fn get_total_user_count(pool: &PgPool) -> sqlx::Result<i64> {
    sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM app_user"#)
        .fetch_one(pool)
        .await
        .inspect_err(|e| error!(error=?e, "fetch total user count failed"))
}
