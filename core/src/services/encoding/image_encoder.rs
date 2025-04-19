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
