use std::io::Write;

use anyhow::Result;
use image::{
    DynamicImage, Frame, ImageFormat,
    codecs::gif::{GifEncoder as InnerGifEncoder, Repeat},
};

use crate::models::options::ProcessingOptions;
use crate::services::encoding::image_encoder::ImageEncoder;

pub struct GifEncoder;

impl ImageEncoder for GifEncoder {
    fn encode(&self, image: &DynamicImage, options: &ProcessingOptions) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.encode_to_writer(&mut buffer, image, options)?;
        Ok(buffer)
    }

    fn encode_to_writer(
        &self,
        writer: &mut dyn Write,
        image: &DynamicImage,
        _options: &ProcessingOptions,
    ) -> Result<()> {
        let rgba_image = image.to_rgba8();
        let frame = Frame::new(rgba_image);
        let mut encoder = InnerGifEncoder::new(writer);
        encoder.set_repeat(Repeat::Finite(0))?;
        encoder.encode_frame(frame)?;
        Ok(())
    }

    fn extension(&self) -> &'static str {
        self.format()
            .extensions_str()
            .first()
            .copied()
            .unwrap_or("gif")
    }

    fn mime_type(&self) -> &'static str {
        self.format().to_mime_type()
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::Gif
    }

    fn supports_native_quality_encoding(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::options::ProcessingOptions;
    use image::DynamicImage;

    fn create_test_image() -> DynamicImage {
        DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(
            4,
            4,
            image::Rgba([255, 0, 0, 255]),
        ))
    }

    mod impl_gif_encoder {
        use super::*;

        #[test]
        fn test_encode_returns_valid_gif_data() {
            let encoder = GifEncoder;
            let image = create_test_image();
            let options = ProcessingOptions::default();

            let result = encoder.encode(&image, &options);
            assert!(result.is_ok());
            let bytes = result.unwrap();
            assert!(!bytes.is_empty());
            // GIF signature: GIF87a or GIF89a
            assert_eq!(&bytes[..3], b"GIF");
        }

        #[test]
        fn test_encode_to_writer_writes_gif_data() {
            let encoder = GifEncoder;
            let image = create_test_image();
            let options = ProcessingOptions::default();

            let mut buffer = Vec::new();
            let result = encoder.encode_to_writer(&mut buffer, &image, &options);
            assert!(result.is_ok());
            assert!(!buffer.is_empty());
            assert_eq!(&buffer[..3], b"GIF");
        }

        #[test]
        fn test_extension_returns_gif() {
            assert_eq!(GifEncoder.extension(), "gif");
        }

        #[test]
        fn test_mime_type_returns_correct_value() {
            assert_eq!(GifEncoder.mime_type(), "image/gif");
        }

        #[test]
        fn test_format_returns_gif_format() {
            assert_eq!(GifEncoder.format(), ImageFormat::Gif);
        }

        #[test]
        fn test_supports_native_quality_encoding_is_false() {
            // GIF uses palette-based compression, no direct quality control
            assert!(!GifEncoder.supports_native_quality_encoding());
        }
    }

    mod gif_encoder {
        use super::*;
        use image::ImageBuffer;

        mod encode {
            use super::*;

            #[test]
            fn encodes_valid_gif_bytes() {
                let encoder = GifEncoder;
                let image = DynamicImage::ImageRgba8(ImageBuffer::from_pixel(
                    32,
                    32,
                    image::Rgba([255, 0, 0, 255]),
                ));
                let opts = ProcessingOptions::default();

                let result = encoder.encode(&image, &opts);
                assert!(result.is_ok());
                let bytes = result.unwrap();
                assert!(!bytes.is_empty(), "Encoded GIF should not be empty");
                assert_eq!(&bytes[..3], b"GIF");
            }

            #[test]
            fn encodes_rgb8_image() {
                let encoder = GifEncoder;
                let image = DynamicImage::ImageRgb8(ImageBuffer::from_pixel(
                    16,
                    16,
                    image::Rgb([0, 255, 0]),
                ));
                let opts = ProcessingOptions::default();

                let result = encoder.encode(&image, &opts);
                assert!(result.is_ok());
                assert!(!result.unwrap().is_empty());
            }

            #[test]
            fn encodes_grayscale_image() {
                let encoder = GifEncoder;
                let image =
                    DynamicImage::ImageLuma8(ImageBuffer::from_pixel(8, 8, image::Luma([128])));
                let opts = ProcessingOptions::default();

                let result = encoder.encode(&image, &opts);
                assert!(result.is_ok());
                assert!(!result.unwrap().is_empty());
            }
        }
    }
}
