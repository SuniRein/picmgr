use sqlx::PgPool;
use tracing::{error, instrument};

#[instrument(skip(pool))]
pub async fn get_total_image_count(pool: &PgPool) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM image
        WHERE trashed_at IS NULL
        "#
    )
    .fetch_one(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch total image count failed"))
}

#[instrument(skip(pool))]
pub async fn get_user_image_count(pool: &PgPool, user_id: i32) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM image
        WHERE trashed_at IS NULL
          AND owner_id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch user image count failed"))
}
