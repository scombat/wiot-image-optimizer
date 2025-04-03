pub use super::resize_options::ResizeOptions;

#[derive(Debug, Default, Clone)]
pub struct ProcessingOptions {
    pub resize: Option<ResizeOptions>,
}

// impl Default for ProcessingOptions {
//     fn default() -> Self {
//         ProcessingOptions {
//             resize: None,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::resize_options::ResizeOptions;

    #[test]
    fn test_default_processing_options() {
        let options = ProcessingOptions::default();
        assert!(options.resize.is_none());
    }

    #[test]
    fn test_processing_options_with_resize() {
        let resize_options = ResizeOptions::default();
        let options = ProcessingOptions {
            resize: Some(resize_options),
        };
        assert!(options.resize.is_some());
    }
}
