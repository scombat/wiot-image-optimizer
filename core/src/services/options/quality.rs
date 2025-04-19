use crate::ImagePipeline;

use anyhow::Result;
use image::ImageFormat;

impl ImagePipeline<'_> {
    pub fn optimize_quality(&mut self) -> Result<(), anyhow::Error> {
        // Step 1: Check if quality options are provided
        let quality_options = match &self.options.quality {
            Some(q) => q,
            None => return Ok(()),
        };

        // Step 2: Check if quality optimization is enabled
        if !quality_options.is_enabled() {
            return Ok(());
        }

        // Step 3: Check if target encoder supports native quality encoding
        let encoder = self.resolve_encoder()?;
        if encoder.supports_native_quality_encoding() {
            return Ok(()); // Native support: let encoder handle it
        }

        // Step 4: Fallback to naive JPEG encoding with quality optimization
        let image = self.get_image()?.clone();
        let jpeg_encoder = self
            .codec_resolver
            .resolve(ImageFormat::Jpeg)
            .ok_or_else(|| {
                anyhow::anyhow!("[core/quality] No JPEG encoder available for quality optimization")
            })?;

        let encoded = jpeg_encoder.encode(&image, &self.options)?;

        // Step 5: Override the image with the processed one
        // This is a workaround for the fact that we don't have a way to set the quality in the encoder
        self.image = Some(image::load_from_memory(&encoded).map_err(|e| {
            anyhow::anyhow!("[core/quality] Failed to load processed image: {}", e)
        })?);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::options::{ProcessingOptions, QualityOptions};
    use crate::services::io::{FileDestination, FileSource};
    use anyhow::Result;
    use async_trait::async_trait;
    use image::{DynamicImage, GenericImageView};
    use std::any::Any;
    use std::sync::Arc;

    struct MockSource;

    #[async_trait]
    impl FileSource for MockSource {
        async fn read(&self, _path: &str) -> Result<DynamicImage> {
            Ok(DynamicImage::new_rgb8(100, 100))
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    struct MockDestination;

    #[async_trait]
    impl FileDestination for MockDestination {
        async fn write(&self, _path: &str, _data: &[u8]) -> Result<()> {
            Ok(())
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    fn create_pipeline(output: &str) -> ImagePipeline<'_> {
        let source = Arc::new(MockSource);
        let destination = Arc::new(MockDestination);
        ImagePipeline::new(source, destination, "input", output)
    }

    fn set_quality_opts(pipeline: &mut ImagePipeline, quality: Option<f32>) {
        pipeline.options = ProcessingOptions {
            quality: Some(QualityOptions::new(quality).unwrap()),
            ..Default::default()
        };
    }

    #[tokio::test]
    async fn test_quality_optimization() {
        let mut pipeline = create_pipeline("output.jpeg");
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(50.0));
        assert!(pipeline.optimize_quality().is_ok());

        let mut pipeline = create_pipeline("output.webp");
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(75.0));
        assert!(pipeline.optimize_quality().is_ok());
    }

    #[tokio::test]
    async fn test_quality_ignored_when_disabled() {
        let mut pipeline = create_pipeline("output.jpeg");
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(100.0));
        assert!(pipeline.optimize_quality().is_ok());
    }

    #[tokio::test]
    async fn test_quality_applied_only_if_image_loaded() {
        let mut pipeline = create_pipeline("output.png");
        set_quality_opts(&mut pipeline, Some(60.0));

        // Do NOT load the image here
        let result = pipeline.optimize_quality();

        assert!(
            result.is_err(),
            "Expected error when optimizing quality without loading image"
        );
    }

    #[tokio::test]
    async fn test_quality_preserves_dimensions_after_optimization() {
        let mut pipeline = create_pipeline("output.jpeg");
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(30.0));
        let original_dims = pipeline.image.as_ref().unwrap().dimensions();
        pipeline.optimize_quality().unwrap();
        let new_dims = pipeline.image.as_ref().unwrap().dimensions();
        assert_eq!(original_dims, new_dims);
    }

    #[tokio::test]
    async fn test_quality_multiple_runs_are_idempotent() {
        let mut pipeline = create_pipeline("output.jpeg");
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(50.0));
        pipeline.optimize_quality().unwrap();
        let first = pipeline.image.as_ref().unwrap().clone();
        pipeline.optimize_quality().unwrap();
        let second = pipeline.image.as_ref().unwrap().clone();
        assert_eq!(first.as_bytes().to_vec(), second.as_bytes().to_vec());
    }
}
