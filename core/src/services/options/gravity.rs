use crate::ImagePipeline;

impl ImagePipeline<'_> {
    pub fn gravity_coords(&self) -> Result<(u32, u32), anyhow::Error> {
        let img = self
            .image
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No image loaded"))?;
        let gravity = self
            .options
            .gravity
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No gravity option set"))?;
        let width = img.width() as f32;
        let height = img.height() as f32;
        let (x_coef, y_coef) = gravity.as_coef();

        let x = (width * x_coef).round() as u32;
        let y = (height * y_coef).round() as u32;
        Ok((x, y))
    }
}

#[cfg(test)]
mod tests {
    use crate::ImagePipeline;
    use crate::models::options::{GravityOptions, ProcessingOptions};
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

    fn set_opts(pipeline: &mut ImagePipeline, gravity: GravityOptions) {
        pipeline.options = ProcessingOptions {
            gravity: Some(gravity),
            resize: None,
            quality: None,
            format: None,
            auto_select_format: false,
            crop: None,
            rotate: None,
            mirror: None,
        };
    }

    #[tokio::test]
    async fn verify_positions() {
        let mut pipeline = create_pipeline();
        let cases = vec![
            (GravityOptions::Top, 50, 0),
            (GravityOptions::TopLeft, 0, 0),
            (GravityOptions::TopRight, 100, 0),
            (GravityOptions::Center, 50, 50),
            (GravityOptions::CenterLeft, 0, 50),
            (GravityOptions::CenterRight, 100, 50),
            (GravityOptions::Bottom, 50, 100),
            (GravityOptions::BottomLeft, 0, 100),
            (GravityOptions::BottomRight, 100, 100),
        ];

        for (gravity, expected_x, expected_y) in cases {
            set_opts(&mut pipeline, gravity);
            pipeline.load().await.unwrap();
            let coords = pipeline.gravity_coords();
            assert!(
                coords.is_ok(),
                "gravity_coords() should return Ok for gravity {:?}",
                gravity
            );
            let (x, y) = coords.unwrap();
            assert_eq!(
                (x, y),
                (expected_x, expected_y),
                "Gravity {:?} expected ({}, {}), got ({}, {})",
                gravity,
                expected_x,
                expected_y,
                x,
                y
            );
        }
    }
}
