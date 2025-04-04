use crate::ImagePipeline;
use crate::services::encoder::get_encoder;

use anyhow::Result;
use image::{DynamicImage, codecs::jpeg::JpegEncoder};
use std::time::Instant;


pub trait QualityEncoder {
    fn encode(&self, image: DynamicImage, quality: u8, input_size: usize) -> Result<(DynamicImage, bool, Option<Vec<u8>>)>;
}

pub struct WebPQualityEncoder;
pub struct JpegQualityEncoder;

impl QualityEncoder for WebPQualityEncoder {
    fn encode(&self, image: DynamicImage, quality: u8, _input_size: usize) -> Result<(DynamicImage, bool, Option<Vec<u8>>)> {
        let rgba = image.to_rgba8();
        let encoder = webp::Encoder::from_rgba(&rgba, rgba.width(), rgba.height());
        let output = encoder.encode(quality as f32);
        let raw_bytes = output.to_vec();
        let processed_image = image::load_from_memory(&raw_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to load processed WebP image: {}", e))?;
        Ok((processed_image, false, Some(raw_bytes)))
    }
}

impl QualityEncoder for JpegQualityEncoder {
    fn encode(&self, image: DynamicImage, quality: u8, _input_size: usize) -> Result<(DynamicImage, bool, Option<Vec<u8>>)> {
        let mut output = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut output, quality);
        encoder.encode_image(&image)
            .map_err(|e| anyhow::anyhow!("Failed to encode JPEG image: {}", e))?;
        let processed_image = image::load_from_memory(&output)
            .map_err(|e| anyhow::anyhow!("Failed to load processed JPEG image: {}", e))?;
        Ok((processed_image, false, Some(output)))
    }
}

pub fn get_encoder_for_format(format: &str) -> Option<Box<dyn QualityEncoder>> {
    match format.to_lowercase().as_str() {
        "webp" => Some(Box::new(WebPQualityEncoder)),
        "jpeg" | "jpg" => Some(Box::new(JpegQualityEncoder)),
        _ => None,
    }
}

impl ImagePipeline<'_> {
    pub fn optimize_quality(&mut self) -> Result<(), anyhow::Error> {
        if let Some(ref quality_options) = self.options.quality {
            if quality_options.is_enabled() {
                let image = self
                    .image
                    .as_mut()
                    .ok_or_else(|| anyhow::anyhow!("[core/quality] Image cannot be loaded"))?;

                let format = quality_options.format.as_deref().unwrap_or("jpeg");
                
                // Get encoder for the format
                let encoder = get_encoder(format)
                    .ok_or_else(|| anyhow::anyhow!("[core/quality] Unsupported format: {}", format))?;

                // Encode the image
                let encoded = encoder.encode(image, &self.options)?;

                // Load the encoded image back
                *image = image::load_from_memory(&encoded)
                    .map_err(|e| anyhow::anyhow!("Failed to load encoded image: {}", e))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;

    fn create_test_image() -> DynamicImage {
        DynamicImage::new_rgb8(100, 100)
    }

    #[test]
    fn test_encoder_resolution() {
        // Test supported formats
        assert!(get_encoder_for_format("webp").is_some());
        assert!(get_encoder_for_format("jpeg").is_some());
        assert!(get_encoder_for_format("jpg").is_some());

        // Test unsupported formats
        assert!(get_encoder_for_format("png").is_none());
        assert!(get_encoder_for_format("gif").is_none());
        assert!(get_encoder_for_format("bmp").is_none());
        assert!(get_encoder_for_format("").is_none());
    }

    #[test]
    fn test_webp_encoder() {
        let encoder = WebPQualityEncoder;
        let image = create_test_image();
        let input_size = 1000; // Dummy size for testing
        let result = encoder.encode(image, 80, input_size);
        
        assert!(result.is_ok());
        let (processed_image, keep_original, raw_bytes) = result.unwrap();
        assert!(!keep_original);
        assert!(raw_bytes.is_some());
        assert_eq!(processed_image.width(), 100);
        assert_eq!(processed_image.height(), 100);
    }

    #[test]
    fn test_jpeg_encoder() {
        let encoder = JpegQualityEncoder;
        let image = create_test_image();
        let input_size = 1000; // Dummy size for testing
        let result = encoder.encode(image, 80, input_size);
        
        assert!(result.is_ok());
        let (processed_image, keep_original, raw_bytes) = result.unwrap();
        assert!(!keep_original);
        assert!(raw_bytes.is_some());
        assert_eq!(processed_image.width(), 100);
        assert_eq!(processed_image.height(), 100);
    }

    #[test]
    fn test_encoder_quality_effects() {
        let image = create_test_image();
        let input_size = 1000; // Dummy size for testing
        
        // Test WebP encoder with different quality settings
        let webp_encoder = WebPQualityEncoder;
        let result_high = webp_encoder.encode(image.clone(), 90, input_size);
        let result_low = webp_encoder.encode(image.clone(), 10, input_size);
        
        assert!(result_high.is_ok());
        assert!(result_low.is_ok());
        
        let (_, _, bytes_high) = result_high.unwrap();
        let (_, _, bytes_low) = result_low.unwrap();
        
        let bytes_high = bytes_high.unwrap();
        let bytes_low = bytes_low.unwrap();
        
        // Both should be smaller than input
        assert!(bytes_high.len() < input_size);
        assert!(bytes_low.len() < input_size);

        // Test JPEG encoder with different quality settings
        let jpeg_encoder = JpegQualityEncoder;
        let result_high = jpeg_encoder.encode(image.clone(), 90, input_size);
        let result_low = jpeg_encoder.encode(image.clone(), 10, input_size);
        
        assert!(result_high.is_ok());
        assert!(result_low.is_ok());
        
        let (_, _, bytes_high) = result_high.unwrap();
        let (_, _, bytes_low) = result_low.unwrap();
        
        let bytes_high = bytes_high.unwrap();
        let bytes_low = bytes_low.unwrap();
        
        // Both should be smaller than input
        assert!(bytes_high.len() < input_size);
        assert!(bytes_low.len() < input_size);
        
        // Lower quality should result in smaller file size for JPEG
        assert!(bytes_low.len() < bytes_high.len());
    }
} 