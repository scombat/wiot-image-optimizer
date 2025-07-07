use std::io::Write;

use crate::services::encoding::image_encoder::{ImageEncoder, encode_to_vec};
use crate::{
    models::options::ProcessingOptions, services::encoding::image_encoder::extract_quality,
};
use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use webp::Encoder as InnerWebPEncoder;

pub struct WebPEncoder;

impl ImageEncoder for WebPEncoder {
    fn encode(&self, image: &DynamicImage, options: &ProcessingOptions) -> Result<Vec<u8>> {
        encode_to_vec(self, image, options)
    }

    fn encode_to_writer(
        &self,
        writer: &mut dyn Write,
        image: &DynamicImage,
        options: &ProcessingOptions,
    ) -> Result<()> {
        let quality = extract_quality(options)?;
        let output = if image.color().has_alpha() {
            let rgba = image.to_rgba8();
            let encoder = InnerWebPEncoder::from_rgba(&rgba, rgba.width(), rgba.height());
            encoder.encode(quality)
        } else {
            let rgb = image.to_rgb8();
            let encoder = InnerWebPEncoder::from_rgb(&rgb, rgb.width(), rgb.height());
            encoder.encode(quality)
        };
        writer.write_all(&output)?;
        Ok(())
    }

    fn extension(&self) -> &'static str {
        self.format()
            .extensions_str()
            .first()
            .copied()
            .unwrap_or("webp")
    }

    fn mime_type(&self) -> &'static str {
        self.format().to_mime_type()
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::WebP
    }

    fn supports_native_quality_encoding(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::options::{ProcessingOptions, QualityOptions};
    use image::DynamicImage;

    fn create_test_image() -> DynamicImage {
        DynamicImage::new_rgba8(32, 32)
    }

    #[test]
    fn test_encode_with_valid_quality() {
        let image = create_test_image();
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(80.0)).unwrap()),
            ..Default::default()
        };
        let encoder = WebPEncoder;

        let result = encoder.encode(&image, &options);
        assert!(result.is_ok(), "WebP encoding should succeed");
        let bytes = result.unwrap();
        assert!(!bytes.is_empty(), "Encoded bytes should not be empty");
    }

    #[test]
    fn test_encode_to_writer_with_valid_quality() {
        let image = create_test_image();
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(60.0)).unwrap()),
            ..Default::default()
        };
        let encoder = WebPEncoder;
        let mut buffer = Vec::new();

        let result = encoder.encode_to_writer(&mut buffer, &image, &options);
        assert!(result.is_ok(), "Writing to buffer should succeed");
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_encode_rgb_image_without_alpha() {
        // Create an RGB image (no alpha channel)
        let image = DynamicImage::new_rgb8(32, 32);
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(85.0)).unwrap()),
            ..Default::default()
        };
        let encoder = WebPEncoder;

        let result = encoder.encode(&image, &options);
        assert!(result.is_ok(), "WebP encoding should succeed for RGB image");
        let bytes = result.unwrap();
        assert!(
            !bytes.is_empty(),
            "Encoded bytes should not be empty for RGB image"
        );
    }

    #[test]
    fn test_encode_rgba_image_with_alpha() {
        // Create an RGBA image (with alpha channel)
        let image = DynamicImage::new_rgba8(32, 32);
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(85.0)).unwrap()),
            ..Default::default()
        };
        let encoder = WebPEncoder;

        let result = encoder.encode(&image, &options);
        assert!(
            result.is_ok(),
            "WebP encoding should succeed for RGBA image"
        );
        let bytes = result.unwrap();
        assert!(
            !bytes.is_empty(),
            "Encoded bytes should not be empty for RGBA image"
        );
    }

    #[test]
    fn test_encode_fails_without_quality() {
        let image = create_test_image();
        let options = ProcessingOptions {
            quality: None,
            ..Default::default()
        };
        let encoder = WebPEncoder;

        let result = encoder.encode(&image, &options);
        assert!(result.is_err(), "Encoding should fail without quality");
    }

    #[test]
    fn test_extension_returns_webp() {
        let encoder = WebPEncoder;
        assert_eq!(encoder.extension(), "webp");
    }

    #[test]
    fn test_mime_type_is_correct() {
        let encoder = WebPEncoder;
        assert_eq!(encoder.mime_type(), "image/webp");
    }

    #[test]
    fn test_format_returns_webp_format() {
        let encoder = WebPEncoder;
        assert_eq!(encoder.format(), ImageFormat::WebP);
    }

    #[test]
    fn test_supports_native_quality_encoding() {
        let encoder = WebPEncoder;
        assert!(encoder.supports_native_quality_encoding());
    }
}
