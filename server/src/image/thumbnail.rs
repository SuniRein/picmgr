use super::storage::{retrieve_with_key, store_with_key};
use image::ImageFormat;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Cursor};
use tracing::{error, instrument};

#[derive(Debug, Clone, Copy, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ThumbnailSize {
    Small,
    Medium,
    Large,
}

impl Display for ThumbnailSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ThumbnailSize::Small => write!(f, "small"),
            ThumbnailSize::Medium => write!(f, "medium"),
            ThumbnailSize::Large => write!(f, "large"),
        }
    }
}

impl ThumbnailSize {
    const fn get_dimensions(&self) -> (u32, u32) {
        match self {
            ThumbnailSize::Small => (150, 150),
            ThumbnailSize::Medium => (300, 300),
            ThumbnailSize::Large => (600, 600),
        }
    }
}

#[instrument(skip(data))]
pub fn generate_thumbnail(data: &[u8], size: ThumbnailSize) -> Result<Vec<u8>, image::ImageError> {
    let img = image::load_from_memory(data)
        .inspect_err(|e| error!(error=?e, "load image from memory failed"))?;

    let (max_width, max_height) = size.get_dimensions();
    let thumbnail = img.thumbnail(max_width, max_height);

    let mut buffer = Cursor::new(Vec::new());
    thumbnail
        .write_to(&mut buffer, ImageFormat::WebP)
        .inspect_err(|e| {
            error!(error=?e, "write thumbnail to buffer failed");
        })?;
    Ok(buffer.into_inner())
}

#[instrument(skip(data, key))]
pub async fn store_thumbnail(data: &[u8], key: &str, size: ThumbnailSize) -> io::Result<()> {
    let thumbnail_key = get_thumbnail_key(key, size);
    store_with_key(&thumbnail_key, data).await
}

#[instrument]
pub async fn retrieve_thumbnail(key: &str, size: ThumbnailSize) -> io::Result<Vec<u8>> {
    let thumbnail_key = get_thumbnail_key(key, size);
    retrieve_with_key(&thumbnail_key).await
}

fn get_thumbnail_key(original_key: &str, size: ThumbnailSize) -> String {
    format!("{original_key}_{size}")
}
