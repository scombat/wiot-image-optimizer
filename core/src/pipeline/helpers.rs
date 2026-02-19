use std::sync::Arc;

use image::{DynamicImage, ImageFormat};

use crate::ImagePipeline;
use crate::services::encoding::image_encoder::ImageEncoder;
use crate::utils::format::infer_format_from_path;

impl ImagePipeline<'_> {
    /*
     * Get format from options
     * If not set, try to infer from the output file extension
     * If not set, try to infer from the input file extension to keep the same format
     */
    fn target_format(&self) -> ImageFormat {
        self.options
            .format
            .as_ref()
            .and_then(|f| f.get_format())
            .or_else(|| infer_format_from_path(&self.output))
            .or_else(|| infer_format_from_path(self.input))
            .unwrap_or(ImageFormat::Jpeg)
    }

    #[allow(clippy::collapsible_if)]
    pub fn resolve_encoder(&self) -> Result<&Arc<dyn ImageEncoder>, anyhow::Error> {
        let target_format = self.target_format();

        if let Some(path_format) = infer_format_from_path(&self.output) {
            if self.options.format.is_some() && path_format != target_format {
                return Err(anyhow::anyhow!(
                    "[core/pipeline] Output format does not match the specified format"
                ));
            }
        }

        self.resolve_encoder_by_format(target_format)
    }

    pub fn resolve_encoder_by_format(
        &self,
        format: ImageFormat,
    ) -> Result<&Arc<dyn ImageEncoder>, anyhow::Error> {
        self.codec_resolver.resolve(format).ok_or_else(|| {
            anyhow::anyhow!(
                "[core/pipeline] No encoder registered for format: {}",
                format.to_mime_type()
            )
        })
    }

    pub fn get_image(&self) -> Result<&DynamicImage, anyhow::Error> {
        self.image
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("[core/pipeline] Image not loaded"))
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::sync::Arc;

    use anyhow::Result;
    use async_trait::async_trait;
    use image::{DynamicImage, ImageFormat};

    use crate::ImagePipeline;
    use crate::models::options::{FormatOptions, ProcessingOptions};
    use crate::services::io::{FileDestination, FileSource};

    struct MockSource;

    #[async_trait]
    impl FileSource for MockSource {
        async fn read(&self, _path: &str) -> Result<DynamicImage> {
            Ok(DynamicImage::new_rgb8(1, 1))
        }
        fn as_any(&self) -> &dyn Any {
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

    fn create_pipeline_with_format(
        output: &str,
        format: Option<FormatOptions>,
    ) -> ImagePipeline<'static> {
        let source = Arc::new(MockSource);
        let destination = Arc::new(MockDestination);
        let mut pipeline = ImagePipeline::new(source, destination, "input.jpg", output.to_string());
        pipeline.options = ProcessingOptions {
            format,
            ..Default::default()
        };
        pipeline
    }

    #[test]
    fn resolve_encoder_errors_on_format_extension_mismatch() {
        let pipeline = create_pipeline_with_format(
            "out.webp",
            Some(FormatOptions::new(Some(ImageFormat::Png)).unwrap()),
        );
        let result = pipeline.resolve_encoder();
        match result {
            Err(e) => assert!(
                e.to_string().contains("does not match"),
                "Expected 'does not match' error, got: {}",
                e
            ),
            Ok(_) => panic!("Expected error for format/extension mismatch"),
        }
    }

    #[test]
    fn resolve_encoder_succeeds_when_format_matches_extension() {
        let pipeline = create_pipeline_with_format(
            "out.webp",
            Some(FormatOptions::new(Some(ImageFormat::WebP)).unwrap()),
        );
        let result = pipeline.resolve_encoder();
        assert!(result.is_ok());
    }

    #[test]
    fn resolve_encoder_falls_back_to_extension_when_no_format() {
        let pipeline = create_pipeline_with_format("out.png", None);
        let result = pipeline.resolve_encoder();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().format(), ImageFormat::Png);
    }
}
