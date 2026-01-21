use std::cmp::max;
use std::io::Write;
use std::thread::available_parallelism;

use anyhow::Result;
use image::ImageEncoder as _;
use image::{DynamicImage, ImageFormat, codecs::avif::AvifEncoder as InnerAvifEncoder};

use crate::models::options::ProcessingOptions;
use crate::services::encoding::image_encoder::{ImageEncoder, encode_to_vec, extract_quality};

pub struct AvifEncoder;

impl ImageEncoder for AvifEncoder {
    fn encode(&self, image: &DynamicImage, options: &ProcessingOptions) -> Result<Vec<u8>> {
        encode_to_vec(self, image, options)
    }

    fn encode_to_writer(
        &self,
        writer: &mut dyn Write,
        image: &DynamicImage,
        options: &ProcessingOptions,
    ) -> Result<()> {
        let quality = extract_quality(options)?;
        let (speed, final_quality, thread_pool_size) = Self::guess_encoding_params(Some(quality));
        let mut encoder = InnerAvifEncoder::new_with_speed_quality(writer, speed, final_quality);
        encoder = encoder.with_num_threads(thread_pool_size);

        if image.color().has_alpha() {
            let buf = image.to_rgba8();
            encoder.write_image(
                &buf,
                buf.width(),
                buf.height(),
                image::ExtendedColorType::Rgba8,
            )?;
        } else {
            let buf = image.to_rgb8();
            encoder.write_image(
                &buf,
                buf.width(),
                buf.height(),
                image::ExtendedColorType::Rgb8,
            )?;
        }
        Ok(())
    }

    fn extension(&self) -> &'static str {
        self.format()
            .extensions_str()
            .first()
            .copied()
            .unwrap_or("avif")
    }

    fn mime_type(&self) -> &'static str {
        self.format().to_mime_type()
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::Avif
    }

    fn supports_native_quality_encoding(&self) -> bool {
        true
    }
}

impl AvifEncoder {
    fn guess_encoding_params(quality: Option<f32>) -> (u8, u8, Option<usize>) {
        let q = quality.unwrap_or(75.0) as u8;

        // Encoding speed is inversely proportional to the requested quality.
        // The higher the quality, the more the AVIF encoder must perform complex
        // analyses and calculations to optimize compression while preserving
        // image details. This requires more processing time.
        let speed = match q {
            81..=100 => 1, // Slowest speed for highest quality
            61..=80 => 3,  // Very slow for high quality
            41..=60 => 5,  // Medium speed for medium quality
            21..=40 => 7,  // Fast for lower quality
            1..=20 => 10,  // Fastest speed for lowest quality
            _ => 5,        // Default to medium speed
        };

        // Determine the number of threads to use for AVIF encoding based on quality.
        // - For high quality (q 61..=100): Use default threading (None) for maximum thread.
        // - For medium quality (q 21..=60): Use half of available threads for a balance of speed and resource usage.
        // - For low quality (q 1..=20): Use a quarter of available threads for fastest encoding with minimal resources.
        // - For any other value: Default to None.
        let thread_pool_size = match q {
            61..=100 => None,
            21..=60 => Some(max(available_parallelism().unwrap().get() / 2, 1)),
            1..=20 => Some(max(available_parallelism().unwrap().get() / 4, 1)),
            _ => None,
        };

        (speed, q, thread_pool_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::options::{ProcessingOptions, QualityOptions};
    use image::DynamicImage;

    fn create_test_image() -> DynamicImage {
        DynamicImage::new_rgba8(32, 32)
    }

    #[test]
    fn test_encode_with_valid_quality() {
        let image = create_test_image();
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(80.0)).unwrap()),
            ..Default::default()
        };
        let encoder = AvifEncoder;
        let result = encoder.encode(&image, &options);
        assert!(result.is_ok(), "Avif encoding should succeed");
        let bytes = result.unwrap();
        assert!(!bytes.is_empty(), "Encoded bytes should not be empty");
    }

    #[test]
    fn test_encode_to_writer_with_valid_quality() {
        let image = create_test_image();
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(60.0)).unwrap()),
            ..Default::default()
        };
        let encoder = AvifEncoder;
        let mut buffer = Vec::new();

        let result = encoder.encode_to_writer(&mut buffer, &image, &options);
        assert!(result.is_ok(), "Writing to buffer should succeed");
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_guess_encoding_params() {
        let available_cores = available_parallelism().unwrap().get();
        let quarter_cores = max(available_cores / 4, 1);
        let half_cores = max(available_cores / 2, 1);

        let cases = vec![
            (1.0, 1u8, 10u8, Some(quarter_cores)),
            (20.0, 20u8, 10u8, Some(quarter_cores)),
            (21.0, 21u8, 7u8, Some(half_cores)),
            (40.0, 40u8, 7u8, Some(half_cores)),
            (41.0, 41u8, 5u8, Some(half_cores)),
            (60.0, 60u8, 5u8, Some(half_cores)),
            (61.0, 61u8, 3u8, None),
            (80.0, 80u8, 3u8, None),
            (81.0, 81u8, 1u8, None),
            (100.0, 100u8, 1u8, None),
        ];

        for (i, case) in cases.iter().enumerate() {
            let (speed, q, thread_pool_size) = AvifEncoder::guess_encoding_params(Some(case.0));
            assert!(
                speed == case.2,
                "Failed to validate speed at {}: {:?}",
                i,
                case
            );
            assert!(
                q == case.1,
                "Failed to validate quality at {}: {:?}",
                i,
                case
            );
            assert!(
                thread_pool_size == case.3,
                "Failed to validate thread pool size at {}: {:?}",
                i,
                case
            );
        }
    }

    #[test]
    fn test_encode_fails_without_quality() {
        let image = create_test_image();
        let options = ProcessingOptions {
            quality: None,
            ..Default::default()
        };
        let encoder = AvifEncoder;

        let result = encoder.encode(&image, &options);
        assert!(result.is_err(), "Encoding should fail without quality");
    }

    #[test]
    fn test_extension_returns_avif() {
        let encoder = AvifEncoder;
        assert_eq!(encoder.extension(), "avif");
    }

    #[test]
    fn test_encode_rgb_image_without_alpha() {
        // Create an RGB image (no alpha channel)
        let image = DynamicImage::new_rgb8(32, 32);
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(85.0)).unwrap()),
            ..Default::default()
        };
        let encoder = AvifEncoder;

        let result = encoder.encode(&image, &options);
        assert!(result.is_ok(), "AVIF encoding should succeed for RGB image");
        let bytes = result.unwrap();
        assert!(
            !bytes.is_empty(),
            "Encoded bytes should not be empty for RGB image"
        );

        // Optionally, check the magic bytes for AVIF (should start with 'ftyp' and 'avif')
        // See: https://github.com/AOMediaCodec/av1-avif/blob/master/avif.md#brand
        assert!(
            bytes
                .windows(8)
                .any(|w| w.starts_with(b"ftyp") && w[4..8] == *b"avif"),
            "Encoded AVIF should contain 'ftypavif' box"
        );
    }

    #[test]
    fn test_encode_rgba_image_with_alpha() {
        // Create an RGBA image (with alpha channel)
        let image = DynamicImage::new_rgba8(32, 32);
        let options = ProcessingOptions {
            quality: Some(QualityOptions::new(Some(85.0)).unwrap()),
            ..Default::default()
        };
        let encoder = AvifEncoder;

        let result = encoder.encode(&image, &options);
        assert!(
            result.is_ok(),
            "AVIF encoding should succeed for RGBA image"
        );
        let bytes = result.unwrap();
        assert!(
            !bytes.is_empty(),
            "Encoded bytes should not be empty for RGBA image"
        );

        // Optionally, check the magic bytes for AVIF (should start with 'ftyp' and 'avif')
        assert!(
            bytes
                .windows(8)
                .any(|w| w.starts_with(b"ftyp") && w[4..8] == *b"avif"),
            "Encoded AVIF should contain 'ftypavif' box"
        );
    }
    #[test]
    fn test_mime_type_is_correct() {
        let encoder = AvifEncoder;
        assert_eq!(encoder.mime_type(), "image/avif");
    }

    #[test]
    fn test_format_returns_avif_format() {
        let encoder = AvifEncoder;
        assert_eq!(encoder.format(), ImageFormat::Avif);
    }

    #[test]
    fn test_supports_native_quality_encoding() {
        let encoder = AvifEncoder;
        assert!(encoder.supports_native_quality_encoding());
    }
}
