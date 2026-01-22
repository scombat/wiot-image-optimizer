use anyhow::Result;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

use crate::ImagePipeline;

impl ImagePipeline<'_> {
    /// Returns true if the current image has an alpha channel
    pub fn image_has_alpha(&self) -> bool {
        self.image
            .as_ref()
            .map(|i| i.color().has_alpha())
            .unwrap_or(false)
    }

    /// Flattens the alpha channel by compositing the image onto a solid background color.
    /// Used when encoding to formats that don't support transparency (e.g., JPEG).
    /// Returns an RGB image (no alpha channel).
    pub fn flatten_alpha(&self) -> Result<DynamicImage> {
        let image = self
            .image
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("[core/alpha] Image not loaded"))?;

        // Get background color from options, default to white
        let bg_color = self
            .options
            .background
            .as_ref()
            .map(|b| b.color)
            .unwrap_or(Rgba([255, 255, 255, 255]));

        let (w, h) = image.dimensions();

        // Create canvas with background color
        let mut canvas = RgbaImage::from_pixel(w, h, bg_color);

        // Overlay the image onto the canvas (handles alpha blending)
        image::imageops::overlay(&mut canvas, &image.to_rgba8(), 0, 0);

        // Convert to RGB (drop alpha since target format doesn't support it)
        Ok(DynamicImage::ImageRgb8(
            DynamicImage::ImageRgba8(canvas).to_rgb8(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use image::{DynamicImage, RgbImage, Rgba, RgbaImage};

    use crate::ImagePipeline;
    use crate::models::background_options::BackgroundOptions;
    use crate::models::options::ProcessingOptions;
    use crate::services::io::{FileDestination, FileSource};

    struct MockSource;
    struct MockDestination;

    #[async_trait::async_trait]
    impl FileSource for MockSource {
        async fn read(&self, _path: &str) -> anyhow::Result<DynamicImage> {
            unimplemented!()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[async_trait::async_trait]
    impl FileDestination for MockDestination {
        async fn write(&self, _path: &str, _data: &[u8]) -> anyhow::Result<()> {
            unimplemented!()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    fn create_pipeline_with_image(image: DynamicImage) -> ImagePipeline<'static> {
        let source = Arc::new(MockSource);
        let destination = Arc::new(MockDestination);
        let mut pipeline =
            ImagePipeline::new(source, destination, "input.png", "output.png".to_string());
        pipeline.image = Some(image);
        pipeline
    }

    fn create_pipeline_with_options(
        image: DynamicImage,
        options: ProcessingOptions,
    ) -> ImagePipeline<'static> {
        let source = Arc::new(MockSource);
        let destination = Arc::new(MockDestination);
        let mut pipeline = ImagePipeline::with_options(
            source,
            destination,
            "input.png",
            "output.png".to_string(),
            options,
        );
        pipeline.image = Some(image);
        pipeline
    }

    mod image_has_alpha {
        use super::*;

        #[test]
        fn returns_true_for_rgba_image() {
            let rgba_image = RgbaImage::from_pixel(10, 10, Rgba([255, 0, 0, 128]));
            let image = DynamicImage::ImageRgba8(rgba_image);
            let pipeline = create_pipeline_with_image(image);

            assert!(pipeline.image_has_alpha());
        }

        #[test]
        fn returns_false_for_rgb_image() {
            let rgb_image = RgbImage::from_pixel(10, 10, image::Rgb([255, 0, 0]));
            let image = DynamicImage::ImageRgb8(rgb_image);
            let pipeline = create_pipeline_with_image(image);

            assert!(!pipeline.image_has_alpha());
        }

        #[test]
        fn returns_false_when_no_image_loaded() {
            let source = Arc::new(MockSource);
            let destination = Arc::new(MockDestination);
            let pipeline =
                ImagePipeline::new(source, destination, "input.png", "output.png".to_string());

            assert!(!pipeline.image_has_alpha());
        }
    }

    mod flatten_alpha {
        use super::*;

        #[test]
        fn with_white_background_produces_correct_result() {
            // Create a 2x2 RGBA image with semi-transparent red
            let mut rgba_image = RgbaImage::new(2, 2);
            // Semi-transparent red (50% alpha)
            rgba_image.put_pixel(0, 0, Rgba([255, 0, 0, 128]));
            rgba_image.put_pixel(1, 0, Rgba([255, 0, 0, 128]));
            rgba_image.put_pixel(0, 1, Rgba([255, 0, 0, 128]));
            rgba_image.put_pixel(1, 1, Rgba([255, 0, 0, 128]));

            let image = DynamicImage::ImageRgba8(rgba_image);
            let pipeline = create_pipeline_with_image(image);

            let result = pipeline.flatten_alpha().unwrap();

            // Result should be RGB (no alpha)
            assert!(!result.color().has_alpha());

            // The color should be a blend of red and white
            let rgb = result.to_rgb8();
            let pixel = rgb.get_pixel(0, 0);
            // Semi-transparent red on white background should produce a pinkish color
            // With 50% alpha: result = 255 * 0.5 + 255 * 0.5 = 255 for red channel
            // and 0 * 0.5 + 255 * 0.5 = 127 for green and blue channels
            assert!(pixel[0] > 200); // Red should be high
            assert!(pixel[1] > 100 && pixel[1] < 150); // Green should be mid-range
            assert!(pixel[2] > 100 && pixel[2] < 150); // Blue should be mid-range
        }

        #[test]
        fn with_custom_background_color() {
            // Create a 2x2 RGBA image with fully transparent pixels
            let rgba_image = RgbaImage::from_pixel(2, 2, Rgba([0, 0, 0, 0]));
            let image = DynamicImage::ImageRgba8(rgba_image);

            // Set custom background color (blue)
            let options = ProcessingOptions {
                background: Some(BackgroundOptions::from_rgb(0, 0, 255)),
                ..Default::default()
            };
            let pipeline = create_pipeline_with_options(image, options);

            let result = pipeline.flatten_alpha().unwrap();

            // Result should be RGB (no alpha)
            assert!(!result.color().has_alpha());

            // With fully transparent image, the result should be the background color
            let rgb = result.to_rgb8();
            let pixel = rgb.get_pixel(0, 0);
            assert_eq!(pixel[0], 0); // Red
            assert_eq!(pixel[1], 0); // Green
            assert_eq!(pixel[2], 255); // Blue
        }

        #[test]
        fn returns_error_when_no_image_loaded() {
            let source = Arc::new(MockSource);
            let destination = Arc::new(MockDestination);
            let pipeline =
                ImagePipeline::new(source, destination, "input.png", "output.png".to_string());

            let result = pipeline.flatten_alpha();
            assert!(result.is_err());
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("[core/alpha] Image not loaded")
            );
        }

        #[test]
        fn preserves_dimensions() {
            let rgba_image = RgbaImage::from_pixel(100, 50, Rgba([255, 0, 0, 128]));
            let image = DynamicImage::ImageRgba8(rgba_image);
            let pipeline = create_pipeline_with_image(image);

            let result = pipeline.flatten_alpha().unwrap();

            assert_eq!(result.width(), 100);
            assert_eq!(result.height(), 50);
        }
    }
}
