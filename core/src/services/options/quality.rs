use crate::ImagePipeline;
use crate::services::encoder::get_encoder;

use anyhow::Result;

impl ImagePipeline<'_> {
    pub fn optimize_quality(&mut self) -> Result<(), anyhow::Error> {
        if let Some(ref quality_options) = self.options.quality {
            if quality_options.is_enabled() {
                let image = self
                    .image
                    .as_mut()
                    .ok_or_else(|| anyhow::anyhow!("[core/quality] Image cannot be loaded"))?;

                // Get format from output file extension
                let format = self.output
                    .split('.')
                    .last()
                    .unwrap_or("jpeg");

                println!("format: {}", format);
                
                // Get encoder for the format
                let encoder = get_encoder(format)
                    .ok_or_else(|| anyhow::anyhow!("[core/quality] Unsupported format: {}", format))?;

                // Encode the image with quality settings
                let encoded = encoder.encode(image, &self.options)?;
                
                // Load the encoded image back
                *image = image::load_from_memory(&encoded)
                    .map_err(|e| anyhow::anyhow!("[core/quality] Failed to load processed image: {}", e))?;
            }
        }
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
    use image::{DynamicImage, ImageFormat};
    use std::sync::Arc;
    use std::any::Any;

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

    fn create_pipeline<'a>(output: &'a str) -> ImagePipeline<'a> {
        let source = Arc::new(MockSource);
        let destination = Arc::new(MockDestination);
        ImagePipeline::new(source, destination, "input", output)
    }

    fn set_quality_opts(pipeline: &mut ImagePipeline, quality: Option<u8>) {
        pipeline.options = ProcessingOptions {
            quality: Some(QualityOptions::new(quality).unwrap()),
            ..Default::default()
        };
    }

    #[tokio::test]
    async fn test_quality_optimization() {
        // Test JPEG quality optimization
        let mut pipeline = create_pipeline("output.jpeg");
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(50));
        assert!(pipeline.optimize_quality().is_ok());

        // Test WebP quality optimization
        let mut pipeline = create_pipeline("output.webp");
        pipeline.load().await.unwrap();
        set_quality_opts(&mut pipeline, Some(75));
        assert!(pipeline.optimize_quality().is_ok());
    }

    #[tokio::test]
    async fn test_quality_optimization_invalid_format() {
        let mut pipeline = create_pipeline("output.png");
        pipeline.image = Some(DynamicImage::new_rgb8(100, 100));
        set_quality_opts(&mut pipeline, Some(80));

        let result = pipeline.optimize_quality();
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unsupported format"), 
               "Expected error about unsupported format, got: {}", err);
    }
} 