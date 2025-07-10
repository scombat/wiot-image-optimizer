use image::ImageFormat;

pub use super::format_options::FormatOptions;
pub use super::quality_options::QualityOptions;
pub use super::resize_options::ResizeOptions;

#[derive(Debug, Default, Clone)]
pub struct ProcessingOptions {
    pub format: Option<FormatOptions>,
    pub quality: Option<QualityOptions>,
    pub resize: Option<ResizeOptions>,
    pub auto_select_format: bool,
}

impl ProcessingOptions {
    pub fn get_quality(&self) -> Option<f32> {
        self.quality.as_ref().and_then(|q| q.quality)
    }

    pub fn get_format(&self) -> Option<ImageFormat> {
        self.format.as_ref().and_then(|f| f.get_format())
    }
}

#[cfg(test)]
mod tests {
    use image::ImageFormat;

    use super::*;
    use crate::models::resize_options::ResizeOptions;

    #[test]
    fn test_default_processing_options() {
        let options = ProcessingOptions::default();
        assert!(options.resize.is_none());
        assert!(options.quality.is_none());
        assert!(options.format.is_none());
    }

    #[test]
    fn test_processing_options_with_resize() {
        let resize_options = ResizeOptions::default();
        let options = ProcessingOptions {
            resize: Some(resize_options),
            ..Default::default()
        };
        assert!(options.resize.is_some());
        assert!(options.quality.is_none());
        assert!(options.format.is_none());
    }

    #[test]
    fn test_processing_options_with_quality() {
        let quality_options = QualityOptions::default();
        let options = ProcessingOptions {
            quality: Some(quality_options),
            ..Default::default()
        };
        assert!(options.resize.is_none());
        assert!(options.quality.is_some());
        assert!(options.format.is_none());
    }

    #[test]
    fn test_processing_options_with_format() {
        let format_options = FormatOptions::new(Some(ImageFormat::WebP));
        let options = ProcessingOptions {
            format: format_options.ok(),
            ..Default::default()
        };
        assert!(options.format.is_some());
        assert!(options.quality.is_none());
        assert!(options.resize.is_none());
    }
}
