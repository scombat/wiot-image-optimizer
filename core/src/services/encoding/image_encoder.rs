use std::io::Write;
use std::sync::Arc;

use anyhow::Result;
use image::{DynamicImage, ImageFormat};

use crate::models::options::ProcessingOptions;

pub trait ImageEncoder: Send + Sync {
    fn encode(&self, image: &DynamicImage, options: &ProcessingOptions) -> Result<Vec<u8>>;
    fn encode_to_writer(
        &self,
        writer: &mut dyn Write,
        image: &DynamicImage,
        options: &ProcessingOptions,
    ) -> Result<()>;

    fn format(&self) -> ImageFormat;
    fn extension(&self) -> &'static str;
    fn mime_type(&self) -> &'static str;
    fn supports_native_quality_encoding(&self) -> bool;
}

pub fn encoder<E: ImageEncoder + 'static>(encoder: E) -> Arc<dyn ImageEncoder> {
    Arc::new(encoder)
}

/// This module defines the `ImageEncoder` trait and related utilities for encoding images
/// into various formats with customizable options such as quality and output destination.
pub fn encode_to_vec<E: ImageEncoder>(
    encoder: &E,
    image: &DynamicImage,
    options: &ProcessingOptions,
) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    encoder.encode_to_writer(&mut buffer, image, options)?;
    Ok(buffer)
}

/// Helper to get the quality value or return an error if not set.
pub fn extract_quality(options: &ProcessingOptions) -> Result<f32> {
    options
        .quality
        .as_ref()
        .and_then(|q| q.quality)
        .ok_or_else(|| anyhow::anyhow!("Quality value is not set in ProcessingOptions"))
}
