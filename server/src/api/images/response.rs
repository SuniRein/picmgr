use chrono::NaiveDateTime;
use serde::Serialize;

use crate::db::image::Image;

#[derive(Debug, Serialize)]
pub struct ImageResponse {
    id: i32,
    owner_id: Option<i32>,
    category_id: Option<i32>,

    size_bytes: i64,
    width: i32,
    height: i32,
    mime_type: String,
    exif: Option<serde_json::Value>,

    is_public: bool,

    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Image> for ImageResponse {
    fn from(image: Image) -> Self {
        ImageResponse {
            id: image.id,
            owner_id: image.owner_id,
            category_id: image.category_id,
            size_bytes: image.size_bytes,
            width: image.width,
            height: image.height,
            mime_type: image.mime_type,
            exif: image.exif,
            is_public: image.is_public,
            created_at: image.created_at,
            updated_at: image.updated_at,
        }
    }
}
