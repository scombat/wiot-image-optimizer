use anyhow::{Result, anyhow};
use image::ImageFormat;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct FormatOptions {
    pub format: Option<ImageFormat>,
}

impl FormatOptions {
    pub fn new(format: Option<ImageFormat>) -> Result<Self> {
        let opts = Self { format };
        opts.validate()?;
        Ok(opts)
    }

    pub fn is_enabled(&self) -> bool {
        self.format.is_some()
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if let Some(format) = self.format {
            if !Self::supported_formats().contains(&format) {
                return Err(anyhow!(
                    "Unsupported output format {:?}. Supported formats: {:?}",
                    format,
                    Self::supported_formats()
                ));
            }
        }

        // If format is None, it's considered valid and let the pipeline with encoder handle it
        Ok(())
    }

    fn supported_formats() -> Vec<ImageFormat> {
        vec![
            ImageFormat::WebP,
            ImageFormat::Avif,
            ImageFormat::Jpeg,
            ImageFormat::Png,
            ImageFormat::Gif,
        ]
    }

    pub fn get_format(&self) -> Option<ImageFormat> {
        self.format
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_format() {
        let opts = FormatOptions::default();
        assert_eq!(opts.format, None);
    }

    #[test]
    fn test_format_validation() {
        assert!(FormatOptions::new(Some(ImageFormat::Jpeg)).is_ok());
    }

    #[test]
    fn test_gif_format_is_supported() {
        assert!(FormatOptions::new(Some(ImageFormat::Gif)).is_ok());
    }

    #[test]
    fn test_is_enabled() {
        let opts = FormatOptions::new(Some(ImageFormat::Png)).unwrap();
        assert!(opts.is_enabled());

        let opts = FormatOptions::new(None).unwrap();
        assert!(!opts.is_enabled());
    }
}
