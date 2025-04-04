use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

use crate::ProcessingOptions;

/// Trait for format-specific image encoding
pub trait ImageEncoder {
    /// Encode the image with the given options
    fn encode(&self, image: &DynamicImage, options: &ProcessingOptions) -> Result<Vec<u8>>;
    
    /// Get the output format for this encoder
    fn format(&self) -> ImageFormat;
}

/// WebP format encoder
pub struct WebPEncoder;

impl ImageEncoder for WebPEncoder {
    fn encode(&self, image: &DynamicImage, options: &ProcessingOptions) -> Result<Vec<u8>> {
        let rgba = image.to_rgba8();
        let quality = options.quality.as_ref()
            .and_then(|q| q.quality)
            .unwrap_or(80) as f32;
            
        let encoder = webp::Encoder::from_rgba(&rgba, rgba.width(), rgba.height());
        let output = encoder.encode(quality);
        Ok(output.to_vec())
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::WebP
    }
}

/// JPEG format encoder
pub struct JpegEncoder;

impl ImageEncoder for JpegEncoder {
    fn encode(&self, image: &DynamicImage, options: &ProcessingOptions) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        let quality = options.quality.as_ref()
            .and_then(|q| q.quality)
            .unwrap_or(80);
            
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
        encoder.encode_image(image)
            .map_err(|e| anyhow::anyhow!("Failed to encode JPEG: {}", e))?;
            
        Ok(output)
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::Jpeg
    }
}

/// Factory function to get an encoder for a specific format
pub fn get_encoder(format: &str) -> Option<Box<dyn ImageEncoder>> {
    match format.to_lowercase().as_str() {
        "webp" => Some(Box::new(WebPEncoder)),
        "jpeg" | "jpg" => Some(Box::new(JpegEncoder)),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_image() -> DynamicImage {
        DynamicImage::new_rgb8(100, 100)
    }
    
    #[test]
    fn test_encoder_resolution() {
        assert!(get_encoder("webp").is_some());
        assert!(get_encoder("jpeg").is_some());
        assert!(get_encoder("jpg").is_some());
        assert!(get_encoder("png").is_none());
    }
    
    #[test]
    fn test_webp_encoding() {
        let encoder = WebPEncoder;
        let image = create_test_image();
        let options = ProcessingOptions::default();
        
        let result = encoder.encode(&image, &options);
        assert!(result.is_ok());
        
        let encoded = result.unwrap();
        assert!(!encoded.is_empty());
    }
    
    #[test]
    fn test_jpeg_encoding() {
        let encoder = JpegEncoder;
        let image = create_test_image();
        let options = ProcessingOptions::default();
        
        let result = encoder.encode(&image, &options);
        assert!(result.is_ok());
        
        let encoded = result.unwrap();
        assert!(!encoded.is_empty());
    }
} 