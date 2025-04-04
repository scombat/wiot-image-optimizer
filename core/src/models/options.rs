pub use super::resize_options::ResizeOptions;
pub use super::quality_options::QualityOptions;

#[derive(Debug, Default, Clone)]
pub struct ProcessingOptions {
    pub resize: Option<ResizeOptions>,
    pub quality: Option<QualityOptions>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::resize_options::ResizeOptions;

    #[test]
    fn test_default_processing_options() {
        let options = ProcessingOptions::default();
        assert!(options.resize.is_none());
        assert!(options.quality.is_none());
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
    }
}
