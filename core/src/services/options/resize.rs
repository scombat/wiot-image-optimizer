use crate::ImagePipeline;

impl ImagePipeline<'_> {
    pub fn resize(&mut self) -> Result<(), anyhow::Error> {
        if let Some(ref resize_options) = self.options.resize {
            if resize_options.is_enabled() {
                let image = self
                    .image
                    .as_mut()
                    .ok_or_else(|| anyhow::anyhow!("[core/resize] Image cannot be loaded"))?;
                let width = resize_options.width.unwrap_or(image.width());
                let height = resize_options.height.unwrap_or(image.height());
                let dpr = resize_options.effective_dpr();

                // Resize the image
                *image = image.resize(
                    (width as f32 * dpr).round() as u32,
                    (height as f32 * dpr).round() as u32,
                    resize_options.filter,
                );
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::ImagePipeline;
    use crate::models::options::{ProcessingOptions, ResizeOptions};
    use crate::services::io::{FileDestination, FileSource};
    use anyhow::Result;
    use async_trait::async_trait;
    use image::{DynamicImage, Rgba};
    use std::sync::Arc;

    /// Mock source that returns a 100x100 white image
    struct MockSource;

    #[async_trait]
    impl FileSource for MockSource {
        async fn read(&self, _path: &str) -> Result<DynamicImage> {
            let image = DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(
                100,
                100,
                Rgba([255, 255, 255, 255]),
            ));
            Ok(image)
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    /// Mock destination that does nothing but captures the image
    struct MockDestination;

    #[async_trait]
    impl FileDestination for MockDestination {
        async fn write(&self, _path: &str, _image: &DynamicImage) -> Result<()> {
            println!("Mock write to {}", _path);
            Ok(())
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    fn create_pipeline() -> ImagePipeline<'static> {
        let source = Arc::new(MockSource);
        let destination = Arc::new(MockDestination);
        ImagePipeline::new(source, destination, "input", "output")
    }

    fn set_opts(pipeline: &mut ImagePipeline, width: u32, height: u32, dpr: f32) {
        pipeline.options = ProcessingOptions {
            resize: Some(ResizeOptions {
                width: Some(width),
                height: Some(height),
                dpr: Some(dpr),
                ..Default::default()
            }),
        };
    }

    async fn verify_base_image_size(pipeline: &mut ImagePipeline<'_>) {
        pipeline.load().await.unwrap();
        assert!(pipeline.image.is_some());
        assert_eq!(pipeline.image.as_ref().unwrap().width(), 100);
        assert_eq!(pipeline.image.as_ref().unwrap().height(), 100);
    }

    #[tokio::test]
    async fn test_pipeline_resize_applies() {
        let mut pipeline = create_pipeline();
        set_opts(&mut pipeline, 50, 50, 1.0);
        verify_base_image_size(&mut pipeline).await;

        // Resize the image & check the size
        pipeline.resize().unwrap();
        assert!(pipeline.image.is_some());
        assert_eq!(pipeline.image.as_ref().unwrap().width(), 50);
        assert_eq!(pipeline.image.as_ref().unwrap().height(), 50);
    }

    #[tokio::test]
    async fn test_pipeline_dpr_resize_applies() {
        let mut pipeline = create_pipeline();
        set_opts(&mut pipeline, 10, 10, 2.0);
        verify_base_image_size(&mut pipeline).await;

        // Resize the image & check the size (10x10 with 2.0 dpr = 20x20)
        pipeline.resize().unwrap();
        assert!(pipeline.image.is_some());
        assert_eq!(pipeline.image.as_ref().unwrap().width(), 20);
        assert_eq!(pipeline.image.as_ref().unwrap().height(), 20);
    }

    #[tokio::test]
    async fn test_pipeline_dpr_resize_round_applies() {
        let mut pipeline = create_pipeline();
        set_opts(&mut pipeline, 10, 10, 2.04);
        verify_base_image_size(&mut pipeline).await;

        // Resize the image & check the size (10x10 with 2.04 dpr should be round to down = 20x20)
        pipeline.resize().unwrap();
        assert!(pipeline.image.is_some());
        assert_eq!(pipeline.image.as_ref().unwrap().width(), 20);
        assert_eq!(pipeline.image.as_ref().unwrap().height(), 20);

        // Resize the image & check the size (10x10 with 2.05 dpr should be round to up = 21x21)
        set_opts(&mut pipeline, 10, 10, 2.05);
        pipeline.resize().unwrap();
        assert!(pipeline.image.is_some());
        assert_eq!(pipeline.image.as_ref().unwrap().width(), 21);
        assert_eq!(pipeline.image.as_ref().unwrap().height(), 21);
    }
}
