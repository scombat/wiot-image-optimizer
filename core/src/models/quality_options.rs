use anyhow::{Result, anyhow};

#[derive(Debug, Clone, PartialEq)]
pub struct QualityOptions {
    pub quality: Option<u8>,
    pub format: Option<String>,
}

impl Default for QualityOptions {
    fn default() -> Self {
        Self {
            quality: Some(80),
            format: None,
        }
    }
}

impl QualityOptions {
    pub fn new(quality: Option<u8>, format: Option<String>) -> Result<Self> {
        let opts = Self {
            quality,
            format,
            ..Self::default()
        };
        opts.validate()?;
        Ok(opts)
    }

    pub fn is_enabled(&self) -> bool {
        self.quality.is_some() || self.format.is_some()
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if let Some(quality) = self.quality {
            if quality > 100 {
                return Err(anyhow!("Quality must be between 0 and 100"));
            }
        }

        if let Some(format) = &self.format {
            match format.to_lowercase().as_str() {
                "jpeg" | "jpg" | "webp" => Ok(()),
                _ => Err(anyhow!("Unsupported format. Supported formats are: jpeg, jpg, webp")),
            }?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_quality() {
        let opts = QualityOptions::default();
        assert_eq!(opts.quality, Some(80));
        assert_eq!(opts.format, None);
    }

    #[test]
    fn test_quality_validation() {
        // Valid quality values
        assert!(QualityOptions::new(Some(0), None).is_ok());
        assert!(QualityOptions::new(Some(50), None).is_ok());
        assert!(QualityOptions::new(Some(100), None).is_ok());

        // Invalid quality values
        assert!(QualityOptions::new(Some(101), None).is_err());
        assert!(QualityOptions::new(Some(255), None).is_err());
    }

    #[test]
    fn test_format_validation() {
        // Valid formats
        assert!(QualityOptions::new(None, Some("jpeg".to_string())).is_ok());
        assert!(QualityOptions::new(None, Some("jpg".to_string())).is_ok());
        assert!(QualityOptions::new(None, Some("webp".to_string())).is_ok());

        // Invalid formats
        assert!(QualityOptions::new(None, Some("png".to_string())).is_err());
        assert!(QualityOptions::new(None, Some("gif".to_string())).is_err());
    }

    #[test]
    fn test_is_enabled() {
        let opts = QualityOptions::new(Some(80), None).unwrap();
        assert!(opts.is_enabled());

        let opts = QualityOptions::new(None, Some("jpeg".to_string())).unwrap();
        assert!(opts.is_enabled());

        let opts = QualityOptions::new(None, None).unwrap();
        assert!(!opts.is_enabled());
    }
} 