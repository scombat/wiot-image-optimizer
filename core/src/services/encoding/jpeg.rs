use crate::{models::options::ProcessingOptions, services::encoding::image_encoder::ImageEncoder};
use anyhow::Result;
use image::{DynamicImage, ImageFormat, codecs::jpeg::JpegEncoder as InnerJpegEncoder};
use std::io::{Cursor, Write};

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
}
