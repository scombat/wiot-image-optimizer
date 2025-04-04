use anyhow::{Result, anyhow};

#[derive(Debug, Clone, PartialEq)]
pub struct QualityOptions {
    pub quality: Option<u8>,
}

impl Default for QualityOptions {
    fn default() -> Self {
        Self {
            quality: Some(80),
        }
    }
}

impl QualityOptions {
    pub fn new(quality: Option<u8>) -> Result<Self> {
        let opts = Self {
            quality,
            ..Self::default()
        };
        opts.validate()?;
        Ok(opts)
    }

    pub fn is_enabled(&self) -> bool {
        self.quality.is_some()
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if let Some(quality) = self.quality {
            if quality > 100 {
                return Err(anyhow!("Quality must be between 0 and 100"));
            }
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
    }

    #[test]
    fn test_quality_validation() {
        // Valid quality values
        assert!(QualityOptions::new(Some(0)).is_ok());
        assert!(QualityOptions::new(Some(50)).is_ok());
        assert!(QualityOptions::new(Some(100)).is_ok());

        // Invalid quality values
        assert!(QualityOptions::new(Some(101)).is_err());
        assert!(QualityOptions::new(Some(255)).is_err());
    }

    #[test]
    fn test_is_enabled() {
        let opts = QualityOptions::new(Some(80)).unwrap();
        assert!(opts.is_enabled());

        let opts = QualityOptions::new(None).unwrap();
        assert!(!opts.is_enabled());
    }
} 