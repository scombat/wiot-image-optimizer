use crate::services::encoding::image_encoder::ImageEncoder;
use image::ImageFormat;
use std::collections::HashMap;
use std::sync::Arc;

pub struct CodecResolver {
    encoders: HashMap<ImageFormat, Arc<dyn ImageEncoder>>,
}

impl CodecResolver {
    pub fn new() -> Self {
        CodecResolver {
            encoders: HashMap::new(),
        }
    }

    pub fn register_encoder(&mut self, format: ImageFormat, encoder: Arc<dyn ImageEncoder>) {
        self.encoders.insert(format, encoder);
    }

    pub fn resolve(&self, format: ImageFormat) -> Option<&Arc<dyn ImageEncoder>> {
        self.encoders.get(&format)
    }

    pub fn supported_formats(&self) -> Vec<ImageFormat> {
        self.encoders.keys().cloned().collect()
    }
}

impl Default for CodecResolver {
    fn default() -> Self {
        let mut resolver = CodecResolver::new();
        use crate::services::encoding::{jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder};
        resolver.register_encoder(ImageFormat::Jpeg, Arc::new(JpegEncoder));
        resolver.register_encoder(ImageFormat::Png, Arc::new(PngEncoder));
        resolver.register_encoder(ImageFormat::WebP, Arc::new(WebPEncoder));

        resolver
    }
}
