use super::super::pagination::DbPagination;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use tracing::{error, instrument};

#[derive(Debug, FromRow, Serialize, utoipa::ToSchema)]
pub struct TrashedImageMeta {
    pub id: i32,
    pub owner_id: Option<i32>,

    pub size_bytes: i64,
    pub width: i32,
    pub height: i32,
    pub mime_type: String,
    pub exif: Option<serde_json::Value>,

    pub is_public: bool,

    pub tags: Vec<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub trashed_at: DateTime<Utc>,
}

#[instrument(skip(pool))]
pub async fn get_user_trashed_image_metas(
    pool: &PgPool,
    owner_id: i32,
    pagination: DbPagination,
) -> sqlx::Result<Vec<TrashedImageMeta>> {
    sqlx::query_as!(
        TrashedImageMeta,
        r#"
        SELECT
          i.id, i.owner_id,
          s.size_bytes, s.width, s.height, s.mime_type, s.exif,
          i.is_public, i.created_at, i.updated_at, i.trashed_at AS "trashed_at!",
          COALESCE(tl.tags, '{}'::text[]) AS "tags!"
        FROM image i
        JOIN image_storage s ON i.storage_id = s.id
        LEFT JOIN LATERAL (
          SELECT ARRAY_AGG(t.name ORDER BY t.name) AS tags
          FROM image_tag it
          JOIN tag t ON it.tag_id = t.id
          WHERE it.image_id = i.id
        ) tl on TRUE
        WHERE i.trashed_at IS NOT NULL
          AND i.owner_id = $1
        ORDER BY i.trashed_at DESC
        LIMIT $2 OFFSET $3
        "#,
        owner_id,
        pagination.limit,
        pagination.offset,
    )
    .fetch_all(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch user trashed image records failed"))
}

#[instrument(skip(pool))]
pub async fn get_user_trashed_image_count(pool: &PgPool, owner_id: i32) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM image
        WHERE trashed_at IS NOT NULL
          AND owner_id = $1
        "#,
        owner_id
    )
    .fetch_one(pool)
    .await
    .inspect_err(|e| error!(error=?e, "fetch user trashed image count failed"))
}

#[instrument(skip(pool))]
pub async fn trash_image(pool: &PgPool, image_id: i32) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        UPDATE image
        SET trashed_at = NOW()
        WHERE trashed_at IS NULL
          AND id = $1
        "#,
        image_id,
    )
    .execute(pool)
    .await
    .inspect_err(|e| error!(error=?e, "trash image failed"))?;
    Ok(())
}

#[instrument(skip(pool))]
pub async fn restore_trashed_image(pool: &PgPool, image_id: i32) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        UPDATE image
        SET trashed_at = NULL
        WHERE trashed_at IS NOT NULL
          AND id = $1
        "#,
        image_id,
    )
    .execute(pool)
    .await
    .inspect_err(|e| error!(error=?e, "restore trashed image failed"))?;
    Ok(())
}
