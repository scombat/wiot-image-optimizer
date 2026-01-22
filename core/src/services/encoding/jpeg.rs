use std::io::{Cursor, Write};

use anyhow::Result;
use image::{DynamicImage, ImageFormat, codecs::jpeg::JpegEncoder as InnerJpegEncoder};

use crate::models::options::ProcessingOptions;
use crate::services::encoding::image_encoder::ImageEncoder;

pub struct JpegEncoder;

impl ImageEncoder for JpegEncoder {
    fn encode(&self, image: &DynamicImage, options: &ProcessingOptions) -> Result<Vec<u8>> {
        let mut buffer = Cursor::new(Vec::new());
        self.encode_to_writer(&mut buffer, image, options)?;
        Ok(buffer.into_inner())
    }

    fn encode_to_writer(
        &self,
        writer: &mut dyn Write,
        image: &DynamicImage,
        options: &ProcessingOptions,
    ) -> Result<()> {
        let quality = options
            .quality
            .as_ref()
            .and_then(|q| q.to_u8())
            .unwrap_or(75);
        let mut encoder = InnerJpegEncoder::new_with_quality(writer, quality);
        encoder.encode_image(image)?;
        Ok(())
    }

    fn extension(&self) -> &'static str {
        self.format()
            .extensions_str()
            .first()
            .copied()
            .unwrap_or("jpg")
    }

    fn mime_type(&self) -> &'static str {
        self.format().to_mime_type()
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::Jpeg
    }

    fn supports_native_quality_encoding(&self) -> bool {
        true
    }

    fn supports_transparency(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::options::{ProcessingOptions, QualityOptions};
    use image::DynamicImage;

    fn create_test_image() -> DynamicImage {
        DynamicImage::new_rgb8(32, 32)
    }

    #[test]
    fn test_encode_with_default_quality() {
        let image = create_test_image();
        let options = ProcessingOptions::default();
        let encoder = JpegEncoder;

        let result = encoder.encode(&image, &options);
        assert!(result.is_ok(), "Encoding should succeed");
        let bytes = result.unwrap();
        assert!(!bytes.is_empty(), "Encoded bytes should not be empty");
    }

    #[test]
    fn test_encode_with_custom_quality() {
        let image = create_test_image();
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(85.0)).unwrap()),
            ..Default::default()
        };
        let encoder = JpegEncoder;

        let result = encoder.encode(&image, &options);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_encode_to_writer_produces_output() {
        let image = create_test_image();
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(60.0)).unwrap()),
            ..Default::default()
        };
        let encoder = JpegEncoder;

        let mut buffer = Vec::new();
        let result = encoder.encode_to_writer(&mut buffer, &image, &options);
        assert!(result.is_ok());
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_extension_is_jpg() {
        let encoder = JpegEncoder;
        assert_eq!(encoder.extension(), "jpg");
    }

    #[test]
    fn test_mime_type_is_image_jpeg() {
        let encoder = JpegEncoder;
        assert_eq!(encoder.mime_type(), "image/jpeg");
    }

    #[test]
    fn test_format_is_jpeg() {
        let encoder = JpegEncoder;
        assert_eq!(encoder.format(), ImageFormat::Jpeg);
    }

    #[test]
    fn test_supports_native_quality_encoding() {
        let encoder = JpegEncoder;
        assert!(encoder.supports_native_quality_encoding());
    }
}
