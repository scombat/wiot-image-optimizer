use std::io::Write;

use crate::models::options::ProcessingOptions;
use anyhow::Result;
use image::ImageEncoder as ImageEncoderTrait;
use image::{
    ColorType, DynamicImage, ImageFormat,
    codecs::png::{CompressionType, FilterType, PngEncoder as InnerPngEncoder},
};

use crate::services::encoding::image_encoder::ImageEncoder;

pub struct PngEncoder;

impl ImageEncoder for PngEncoder {
    fn encode(&self, image: &DynamicImage, _options: &ProcessingOptions) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.encode_to_writer(&mut buffer, image, _options)?;
        Ok(buffer)
    }

    fn encode_to_writer(
        &self,
        writer: &mut dyn Write,
        image: &DynamicImage,
        options: &ProcessingOptions,
    ) -> Result<()> {
        let (compression, filter) = PngEncoder::guess_encoding_params(options.get_quality());
        let (buffer, width, height, color_type) = PngEncoder::extract_buffer_and_type(image)?;
        let encoder = InnerPngEncoder::new_with_quality(writer, compression, filter);
        encoder.write_image(buffer, width, height, color_type.into())?;
        Ok(())
    }

    fn extension(&self) -> &'static str {
        self.format()
            .extensions_str()
            .first()
            .copied()
            .unwrap_or("png")
    }

    fn mime_type(&self) -> &'static str {
        self.format().to_mime_type()
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::Png
    }

    fn supports_native_quality_encoding(&self) -> bool {
        false
    }
}

impl PngEncoder {
    fn guess_encoding_params(quality: Option<f32>) -> (CompressionType, FilterType) {
        // Map quality to PNG compression strategy:
        // - High quality → Best compression (slow, small file)
        // - Medium quality → Default
        // - Low quality → Fast (quick, larger file)
        let compression = match quality.unwrap_or(75.0).round() as u8 {
            90..=100 => CompressionType::Best,
            40..=89 => CompressionType::Default,
            _ => CompressionType::Fast,
        };

        let filter = FilterType::Adaptive;
        (compression, filter)
    }

    pub fn extract_buffer_and_type(
        image: &DynamicImage,
    ) -> Result<(&[u8], u32, u32, ColorType), anyhow::Error> {
        match image {
            DynamicImage::ImageLuma8(buf) => {
                Ok((buf.as_raw(), buf.width(), buf.height(), ColorType::L8))
            }
            DynamicImage::ImageLumaA8(buf) => {
                Ok((buf.as_raw(), buf.width(), buf.height(), ColorType::La8))
            }
            DynamicImage::ImageRgb8(buf) => {
                Ok((buf.as_raw(), buf.width(), buf.height(), ColorType::Rgb8))
            }
            DynamicImage::ImageRgba8(buf) => {
                Ok((buf.as_raw(), buf.width(), buf.height(), ColorType::Rgba8))
            }
            DynamicImage::ImageLuma16(buf) => Ok((
                Self::as_u8_slice(buf),
                buf.width(),
                buf.height(),
                ColorType::L16,
            )),
            DynamicImage::ImageLumaA16(buf) => Ok((
                Self::as_u8_slice(buf),
                buf.width(),
                buf.height(),
                ColorType::La16,
            )),
            DynamicImage::ImageRgb16(buf) => Ok((
                Self::as_u8_slice(buf),
                buf.width(),
                buf.height(),
                ColorType::Rgb16,
            )),
            DynamicImage::ImageRgba16(buf) => Ok((
                Self::as_u8_slice(buf),
                buf.width(),
                buf.height(),
                ColorType::Rgba16,
            )),
            _ => Err(anyhow::anyhow!(
                "Unsupported DynamicImage variant for PNG encoding"
            )),
        }
    }

    /// For 16-bit buffers, convert &[u16] to &[u8]
    fn as_u8_slice<T: bytemuck::Pod>(buffer: &[T]) -> &[u8] {
        bytemuck::cast_slice(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::codecs::png::{CompressionType, FilterType};
    use image::{DynamicImage, ImageBuffer};

    fn create_test_image() -> DynamicImage {
        DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(
            4,
            4,
            image::Rgba([255, 0, 0, 255]),
        ))
    }

    mod impl_png_encoder {
        use super::*;

        #[test]
        fn test_encode_returns_valid_png_data() {
            let encoder = PngEncoder;
            let image = create_test_image();
            let options = ProcessingOptions::default();

            let result = encoder.encode(&image, &options);
            assert!(result.is_ok());
            let bytes = result.unwrap();
            assert!(!bytes.is_empty());
            assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n"); // PNG signature
        }

        #[test]
        fn test_encode_to_writer_writes_png_data() {
            let encoder = PngEncoder;
            let image = create_test_image();
            let options = ProcessingOptions::default();

            let mut buffer = Vec::new();
            let result = encoder.encode_to_writer(&mut buffer, &image, &options);
            assert!(result.is_ok());
            assert!(!buffer.is_empty());
            assert_eq!(&buffer[..8], b"\x89PNG\r\n\x1a\n"); // PNG signature
        }

        #[test]
        fn test_extension_returns_png() {
            let encoder = PngEncoder;
            assert_eq!(encoder.extension(), "png");
        }

        #[test]
        fn test_mime_type_returns_correct_value() {
            let encoder = PngEncoder;
            assert_eq!(encoder.mime_type(), "image/png");
        }

        #[test]
        fn test_format_returns_png_format() {
            let encoder = PngEncoder;
            assert_eq!(encoder.format(), ImageFormat::Png);
        }

        #[test]
        fn test_supports_native_quality_encoding_is_false() {
            let encoder = PngEncoder;
            assert!(!encoder.supports_native_quality_encoding());
        }
    }

    mod png_encoder {
        use super::*;
        mod guess_encoding {
            use super::*;

            #[test]
            fn for_quality_levels() {
                let test_cases = vec![
                    (Some(95.0), CompressionType::Best),
                    (Some(90.0), CompressionType::Best),
                    (Some(89.0), CompressionType::Default),
                    (Some(75.0), CompressionType::Default),
                    (Some(40.0), CompressionType::Default),
                    (Some(39.0), CompressionType::Fast),
                    (Some(10.0), CompressionType::Fast),
                    (None, CompressionType::Default), // fallback quality = 75
                ];

                for (quality, expected_compression) in test_cases {
                    let (compression, filter) = PngEncoder::guess_encoding_params(quality);
                    assert_eq!(
                        compression, expected_compression,
                        "Failed for quality: {:?}",
                        quality
                    );
                    assert_eq!(
                        filter,
                        FilterType::Adaptive,
                        "Filter should always be Adaptive"
                    );
                }
            }
        }

        mod extract_buffer {
            use super::*;
            #[test]
            fn all_supported_variants_are_handled() {
                use image::{ImageBuffer, Luma, LumaA, Rgb, Rgba};

                let variants = vec![
                    DynamicImage::ImageLuma8(ImageBuffer::from_pixel(1, 1, Luma([0]))),
                    DynamicImage::ImageLumaA8(ImageBuffer::from_pixel(1, 1, LumaA([0, 0]))),
                    DynamicImage::ImageRgb8(ImageBuffer::from_pixel(1, 1, Rgb([0, 0, 0]))),
                    DynamicImage::ImageRgba8(ImageBuffer::from_pixel(1, 1, Rgba([0, 0, 0, 0]))),
                    DynamicImage::ImageLuma16(ImageBuffer::from_pixel(1, 1, Luma([0u16]))),
                    DynamicImage::ImageLumaA16(ImageBuffer::from_pixel(1, 1, LumaA([0u16, 0u16]))),
                    DynamicImage::ImageRgb16(ImageBuffer::from_pixel(
                        1,
                        1,
                        Rgb([0u16, 0u16, 0u16]),
                    )),
                    DynamicImage::ImageRgba16(ImageBuffer::from_pixel(
                        1,
                        1,
                        Rgba([0u16, 0u16, 0u16, 0u16]),
                    )),
                ];

                for variant in variants {
                    let result = PngEncoder::extract_buffer_and_type(&variant);
                    assert!(
                        result.is_ok(),
                        "Variant should be supported: {:?}",
                        variant.color()
                    );
                }
            }
        }

        mod encode {
            use super::*;

            #[test]
            fn encodes_valid_png_bytes() {
                let encoder = PngEncoder;
                let image = DynamicImage::ImageRgba8(ImageBuffer::from_pixel(
                    32,
                    32,
                    image::Rgba([255, 0, 0, 255]),
                ));
                let opts = ProcessingOptions::default();

                let result = encoder.encode(&image, &opts);
                assert!(result.is_ok());
                let bytes = result.unwrap();
                assert!(!bytes.is_empty(), "Encoded PNG should not be empty");
                assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
            }
        }

        mod as_u8_slice {
            use super::*;
            #[test]
            fn test_as_u8_slice_with_u16_buffer() {
                let u16_data: &[u16] = &[0x1234, 0x5678, 0xABCD];
                let u8_slice = PngEncoder::as_u8_slice(u16_data);

                assert_eq!(u8_slice.len(), u16_data.len() * 2);

                let expected_bytes: Vec<u8> =
                    u16_data.iter().flat_map(|x| x.to_ne_bytes()).collect();
                assert_eq!(u8_slice, expected_bytes.as_slice());
            }

            #[test]
            fn test_as_u8_slice_empty() {
                let empty: &[u16] = &[];
                let bytes = PngEncoder::as_u8_slice(empty);
                assert_eq!(bytes.len(), 0);
                assert!(bytes.is_empty());
            }
        }
    }
}
