use image::{DynamicImage, RgbaImage, imageops::overlay};

use crate::{ImagePipeline, models::resize_options::AspectRatioStrategy};

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
                let dpr = resize_options.dpr;

                let target_w = (width as f32 * dpr).round() as u32;
                let target_h = (height as f32 * dpr).round() as u32;
                let filter = resize_options.filter;

                let new_img: DynamicImage = match resize_options.strategy {
                    AspectRatioStrategy::Fit => image.resize(target_w, target_h, filter),
                    AspectRatioStrategy::Cover => image.resize_to_fill(target_w, target_h, filter),
                    AspectRatioStrategy::Stretch => image.resize_exact(target_w, target_h, filter),
                    AspectRatioStrategy::Contain => {
                        let scaled = image.resize(target_w, target_h, filter);
                        let (scaled_w, scaled_h) = (scaled.width(), scaled.height());
                        let x = (target_w.saturating_sub(scaled_w)) / 2;
                        let y = (target_h.saturating_sub(scaled_h)) / 2;

                        if image.color().has_alpha() {
                            let mut shape = RgbaImage::new(target_w, target_h);
                            overlay(&mut shape, &scaled, x.into(), y.into());
                            DynamicImage::ImageRgba8(shape)
                        } else {
                            let mut shape = image::RgbImage::new(target_w, target_h);
                            overlay(&mut shape, &scaled.to_rgb8(), x.into(), y.into());
                            DynamicImage::ImageRgb8(shape)
                        }
                    }
                };

                *image = new_img;
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
    use std::any::Any;
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
        async fn write(&self, _path: &str, _data: &[u8]) -> Result<()> {
            Ok(())
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    fn create_pipeline() -> ImagePipeline<'static> {
        let source = Arc::new(MockSource);
        let destination = Arc::new(MockDestination);
        ImagePipeline::new(source, destination, "input", "output".to_string())
    }

    fn set_opts(
        pipeline: &mut ImagePipeline,
        width: Option<u32>,
        height: Option<u32>,
        dpr: Option<f32>,
    ) {
        pipeline.options = ProcessingOptions {
            resize: Some(ResizeOptions {
                width,
                height,
                dpr: dpr.unwrap_or(1.0),
                ..Default::default()
            }),
            quality: None,
            format: None,
            auto_select_format: false,
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
        set_opts(&mut pipeline, Some(50), Some(50), Some(1.0));
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
        set_opts(&mut pipeline, Some(10), Some(10), Some(2.0));
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
        set_opts(&mut pipeline, Some(10), Some(10), Some(2.04));
        verify_base_image_size(&mut pipeline).await;

        // Resize the image & check the size (10x10 with 2.04 dpr should be round to down = 20x20)
        pipeline.resize().unwrap();
        assert!(pipeline.image.is_some());
        assert_eq!(pipeline.image.as_ref().unwrap().width(), 20);
        assert_eq!(pipeline.image.as_ref().unwrap().height(), 20);

        // Resize the image & check the size (10x10 with 2.05 dpr should be round to up = 21x21)
        set_opts(&mut pipeline, Some(10), Some(10), Some(2.05));
        pipeline.resize().unwrap();
        assert!(pipeline.image.is_some());
        assert_eq!(pipeline.image.as_ref().unwrap().width(), 21);
        assert_eq!(pipeline.image.as_ref().unwrap().height(), 21);
    }

    #[tokio::test]
    async fn test_pipeline_dpr_without_resize_options() {
        let mut pipeline = create_pipeline();
        set_opts(&mut pipeline, None, None, Some(2.0));
        verify_base_image_size(&mut pipeline).await;

        // Resize the image & check the size (100x100 with 2.0 dpr = 20x20)
        pipeline.resize().unwrap();
        assert!(pipeline.image.is_some());
        assert_eq!(pipeline.image.as_ref().unwrap().width(), 200);
        assert_eq!(pipeline.image.as_ref().unwrap().height(), 200);
    }

    #[tokio::test]
    async fn test_pipeline_resize_and_encode_with_alpha() {
        use crate::models::options::ProcessingOptions;
        use crate::services::encoding::image_encoder::ImageEncoder;
        use crate::services::encoding::png::PngEncoder;
        use image::DynamicImage;

        let mut pipeline = create_pipeline();
        // Set a small resize with alpha (RGBA)
        set_opts(&mut pipeline, Some(32), Some(32), None);
        verify_base_image_size(&mut pipeline).await;

        pipeline.resize().unwrap();
        assert!(pipeline.image.is_some());
        let image = pipeline.image.as_ref().unwrap();

        // Ensure the image has alpha
        assert!(matches!(image, DynamicImage::ImageRgba8(_)));

        // Try encoding with PNG (supports alpha)
        let encoder = PngEncoder;
        let options = ProcessingOptions::default();
        let encoded = encoder.encode(image, &options);
        assert!(encoded.is_ok(), "PNG encoding with alpha should succeed");
        let bytes = encoded.unwrap();
        assert!(!bytes.is_empty(), "Encoded PNG bytes should not be empty");
    }

    #[tokio::test]
    async fn test_pipeline_resize_and_encode_without_alpha() {
        use crate::models::options::ProcessingOptions;
        use crate::services::encoding::image_encoder::ImageEncoder;
        use crate::services::encoding::png::PngEncoder;
        use image::DynamicImage;

        let mut pipeline = create_pipeline();
        // Set a small resize and force RGB (no alpha)
        set_opts(&mut pipeline, Some(32), Some(32), None);
        verify_base_image_size(&mut pipeline).await;

        pipeline.resize().unwrap();
        assert!(pipeline.image.is_some());
        let image = pipeline.image.as_ref().unwrap().to_rgb8();
        let image = DynamicImage::ImageRgb8(image);

        // Ensure the image does not have alpha
        assert!(matches!(image, DynamicImage::ImageRgb8(_)));

        // Try encoding with PNG (no alpha)
        let encoder = PngEncoder;
        let options = ProcessingOptions::default();
        let encoded = encoder.encode(&image, &options);
        assert!(encoded.is_ok(), "PNG encoding without alpha should succeed");
        let bytes = encoded.unwrap();
        assert!(!bytes.is_empty(), "Encoded PNG bytes should not be empty");
    }
}
