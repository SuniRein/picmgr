use image::{GenericImageView, ImageError, ImageFormat};
use tracing::{error, instrument};

#[derive(Debug)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub mime_type: String,
    pub exif: Option<serde_json::Value>,
}

impl ImageInfo {
    #[instrument(skip(bytes))]
    pub fn parse(bytes: &[u8]) -> Result<Self, ImageParseError> {
        let format = image::guess_format(bytes)?;
        let mime_type = match format {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Bmp => "image/bmp",
            ImageFormat::Ico => "image/x-icon",
            ImageFormat::Tiff => "image/tiff",
            ImageFormat::WebP => "image/webp",
            _ => {
                error!("Unsupported image format: {:?}", format);
                return Err(ImageParseError::UnsupportedFormat);
            }
        }
        .to_string();

        let img = image::load_from_memory_with_format(bytes, format)
            .inspect_err(|e| error!("Failed to parse image: {:?}", e))?;
        let (width, height) = img.dimensions();

        Ok(ImageInfo {
            width,
            height,
            mime_type,
            exif: None, // TODO: extract EXIF data if needed
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ImageParseError {
    #[error("Unsupported Image Format")]
    UnsupportedFormat,

    #[error("Image Parse Error")]
    ParseError,
}

impl From<ImageError> for ImageParseError {
    fn from(err: ImageError) -> Self {
        match err {
            ImageError::Unsupported(_) => ImageParseError::UnsupportedFormat,
            _ => ImageParseError::ParseError,
        }
    }
}
