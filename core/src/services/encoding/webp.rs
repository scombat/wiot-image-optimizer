use std::io::Write;

use crate::models::options::ProcessingOptions;
use crate::services::encoding::image_encoder::ImageEncoder;
use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use webp::Encoder as InnerWebPEncoder;

pub struct WebPEncoder;

impl ImageEncoder for WebPEncoder {
    fn encode(&self, image: &DynamicImage, options: &ProcessingOptions) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.encode_to_writer(&mut buffer, image, options)?;
        Ok(buffer)
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
            .and_then(|q| q.quality)
            .ok_or_else(|| anyhow::anyhow!("Quality value is not set in ProcessingOptions"))?;
        let rgba = image.to_rgba8();
        let encoder = InnerWebPEncoder::from_rgba(&rgba, rgba.width(), rgba.height());
        let output = encoder.encode(quality);
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
