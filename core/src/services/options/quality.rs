use anyhow::Result;
use image::ImageFormat;

use crate::ImagePipeline;

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

        // Get the image early to fail fast if it's not loaded, before doing expensive operations
        let image = self.get_image()?;

        // Step 3: Check if target encoder supports native quality encoding
        let encoder = self.resolve_encoder()?;
        if encoder.supports_native_quality_encoding() {
            return Ok(()); // Native support: let encoder handle it
        }

        // Step 4: Fallback to naive JPEG encoding with quality optimization
        let jpeg_encoder = self
            .codec_resolver
            .resolve(ImageFormat::Jpeg)
            .ok_or_else(|| {
                anyhow::anyhow!("[core/quality] No JPEG encoder available for quality optimization")
            })?;

        let encoded = jpeg_encoder.encode(&image.clone(), &self.options)?;

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
    use crate::services::encoding::codec_resolver::CodecResolver;
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

    fn create_pipeline(output: String) -> ImagePipeline<'static> {
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
        let mut pipeline = create_pipeline("output.jpeg".to_string());
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(50.0));
        assert!(pipeline.optimize_quality().is_ok());

        let mut pipeline = create_pipeline("output.webp".to_string());
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(75.0));
        assert!(pipeline.optimize_quality().is_ok());
    }

    #[tokio::test]
    async fn test_quality_skipped_when_no_quality_option() {
        let mut pipeline = create_pipeline("output.jpeg".to_string());
        pipeline.load().await.unwrap();

        // No quality set at all
        pipeline.options.quality = None;
        let result = pipeline.optimize_quality();
        assert!(
            result.is_ok(),
            "Should silently skip when no quality options"
        );
    }

    #[tokio::test]
    async fn test_quality_ignored_when_disabled() {
        let mut pipeline = create_pipeline("output.jpeg".to_string());
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(100.0));
        assert!(pipeline.optimize_quality().is_ok());
    }

    #[tokio::test]
    async fn test_quality_applied_only_if_image_loaded() {
        let mut pipeline = create_pipeline("output.png".to_string());
        set_quality_opts(&mut pipeline, Some(60.0));

        // Do NOT load the image here
        let result = pipeline.optimize_quality();

        assert!(
            result.is_err(),
            "Expected error when optimizing quality without loading image"
        );
    }

    #[tokio::test]
    async fn test_quality_skipped_when_encoder_handles_quality() {
        let mut pipeline = create_pipeline("output.webp".to_string()); // WebP supports native quality
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(70.0));

        let result = pipeline.optimize_quality();
        assert!(
            result.is_ok(),
            "Should skip optimization if encoder handles quality natively"
        );
    }

    #[tokio::test]
    async fn test_quality_preserves_dimensions_after_optimization() {
        let mut pipeline = create_pipeline("output.jpeg".to_string());
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(30.0));
        let original_dims = pipeline.image.as_ref().unwrap().dimensions();
        pipeline.optimize_quality().unwrap();
        let new_dims = pipeline.image.as_ref().unwrap().dimensions();
        assert_eq!(original_dims, new_dims);
    }

    #[tokio::test]
    async fn test_quality_multiple_runs_are_idempotent() {
        let mut pipeline = create_pipeline("output.jpeg".to_string());
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(50.0));
        pipeline.optimize_quality().unwrap();
        let first = pipeline.image.as_ref().unwrap().clone();
        pipeline.optimize_quality().unwrap();
        let second = pipeline.image.as_ref().unwrap().clone();
        assert_eq!(first.as_bytes().to_vec(), second.as_bytes().to_vec());
    }

    #[tokio::test]
    async fn test_quality_error_when_no_jpeg_encoder_available() {
        let mut pipeline = create_pipeline("output.unknown".to_string()); // Unknown extension forces fallback
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(80.0));

        // Remove JPEG encoder from registry
        pipeline.codec_resolver = CodecResolver::new();
        pipeline.codec_resolver.register_encoder(
            ImageFormat::WebP,
            Arc::new(crate::services::encoding::webp::WebPEncoder),
        );

        let result = pipeline.optimize_quality();
        assert!(
            result.is_err(),
            "Should return error if no JPEG encoder is found during fallback"
        );
    }

    #[tokio::test]
    async fn test_quality_error_on_invalid_encoded_image() {
        let mut pipeline = create_pipeline("output.jpeg".to_string());
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(60.0));

        use crate::services::encoding::image_encoder::ImageEncoder;
        use std::sync::Arc;

        struct BrokenEncoder;
        impl ImageEncoder for BrokenEncoder {
            fn encode(
                &self,
                _image: &DynamicImage,
                _options: &ProcessingOptions,
            ) -> Result<Vec<u8>> {
                Ok(vec![0u8; 10]) // Invalid data
            }
            fn encode_to_writer(
                &self,
                _: &mut dyn std::io::Write,
                _: &DynamicImage,
                _: &ProcessingOptions,
            ) -> Result<()> {
                Ok(())
            }
            fn extension(&self) -> &'static str {
                "jpg"
            }
            fn mime_type(&self) -> &'static str {
                "image/jpeg"
            }
            fn format(&self) -> ImageFormat {
                ImageFormat::Jpeg
            }
            fn supports_native_quality_encoding(&self) -> bool {
                false
            }
        }

        pipeline
            .codec_resolver
            .register_encoder(ImageFormat::Jpeg, Arc::new(BrokenEncoder));

        let result = pipeline.optimize_quality();
        assert!(
            result.is_err(),
            "Should fail if optimized image is not decodable"
        );
    }
}
