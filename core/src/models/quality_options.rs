use anyhow::{Result, anyhow};

#[derive(Debug, Clone, PartialEq)]
pub struct QualityOptions {
    pub quality: Option<f32>,
}

impl Default for QualityOptions {
    fn default() -> Self {
        Self {
            quality: Some(75.0),
        }
    }
}

impl QualityOptions {
    pub fn new(quality: Option<f32>) -> Result<Self> {
        let opts = Self { quality };
        opts.validate()?;
        Ok(opts)
    }

    pub fn is_enabled(&self) -> bool {
        self.quality.is_some()
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if let Some(quality) = self.quality {
            if quality > 100.0 {
                return Err(anyhow!("Quality must be between 0 and 100"));
            }
        }
        Ok(())
    }

    pub fn to_u8(&self) -> Option<u8> {
        self.quality.map(|q| q as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_quality() {
        let opts = QualityOptions::default();
        assert_eq!(opts.quality, Some(75.0));
    }

    #[test]
    fn test_quality_validation() {
        // Valid quality values
        assert!(QualityOptions::new(Some(0.0)).is_ok());
        assert!(QualityOptions::new(Some(50.0)).is_ok());
        assert!(QualityOptions::new(Some(100.0)).is_ok());

        // Invalid quality values
        assert!(QualityOptions::new(Some(101.0)).is_err());
        assert!(QualityOptions::new(Some(255.0)).is_err());
    }

    #[test]
    fn test_is_enabled() {
        let opts = QualityOptions::new(Some(80.0)).unwrap();
        assert!(opts.is_enabled());

        let opts = QualityOptions::new(None).unwrap();
        assert!(!opts.is_enabled());
    }

    #[test]
    fn to_u8() {
        let opts = QualityOptions::new(Some(80.0)).unwrap();
        assert_eq!(opts.to_u8(), Some(80));

        let opts = QualityOptions::new(None).unwrap();
        assert_eq!(opts.to_u8(), None);
    }
}
