use exif::Reader as ExifReader;
use image::{ImageDecoder, ImageError, ImageFormat, ImageReader};
use std::io::Cursor;
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
                error!(?format, "image format unsupported");
                return Err(ImageParseError::UnsupportedFormat);
            }
        }
        .to_string();

        let mut img_decoder = ImageReader::with_format(Cursor::new(bytes), format)
            .into_decoder()
            .inspect_err(|e| error!(?e, "image parse failed"))?;
        let (width, height) = img_decoder.dimensions();

        let exif_raw = img_decoder
            .exif_metadata()
            .inspect_err(|e| error!(?e, "read EXIF metadata failed"))
            .ok()
            .flatten();
        let exif_reader = ExifReader::new();
        let exif = match exif_raw {
            Some(data) => match exif_reader.read_raw(data) {
                Ok(exif) => {
                    let mut exif_map = serde_json::Map::new();
                    for field in exif.fields() {
                        exif_map.insert(
                            field.tag.to_string(),
                            serde_json::Value::String(
                                field.display_value().with_unit(&exif).to_string(),
                            ),
                        );
                    }
                    Some(serde_json::Value::Object(exif_map))
                }
                Err(e) => {
                    error!(?e, "parse EXIF data failed");
                    None
                }
            },
            None => None,
        };

        Ok(ImageInfo {
            width,
            height,
            mime_type,
            exif,
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
