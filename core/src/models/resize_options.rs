use anyhow::{Result, anyhow};
use image::imageops::FilterType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResizeOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub dpr: Option<f32>,
    pub filter: FilterType,
}

impl Default for ResizeOptions {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            dpr: None,
            filter: FilterType::Triangle,
        }
    }
}

impl ResizeOptions {
    pub fn new(width: Option<u32>, height: Option<u32>, dpr: Option<f32>) -> Result<Self> {
        let opts: ResizeOptions = Self {
            width,
            height,
            dpr,
            ..Self::default()
        };
        opts.validate()?;
        Ok(opts)
    }

    pub fn is_enabled(&self) -> bool {
        self.width.is_some() || self.height.is_some()
    }

    pub fn effective_dpr(&self) -> f32 {
        self.dpr.unwrap_or(1.0)
    }

    /*
     * Validate the resize options.
     * - Width and height must be greater than 0 if specified.
     * - DPR must be between 0.1 and 10.0 if specified.
     * - If both width and height are specified, they must be greater than 0.
     * - If only one of width or height is specified, the other must be None.
     * - If both width and height are None, the resize options are considered valid.
     */
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if self.width.is_some() && self.width.unwrap() == 0 {
            return Err(anyhow!("Resize width must be greater than 0"));
        }

        if self.height.is_some() && self.height.unwrap() == 0 {
            return Err(anyhow!("Resize height must be greater than 0"));
        }

        if let Some(dpr) = self.dpr {
            if !(0.1..=10.0).contains(&dpr) {
                return Err(anyhow!("Resize dpr must be between 0.1 and 10.0"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_resize_options_pass() {
        let opts = ResizeOptions {
            width: Some(800),
            height: Some(600),
            dpr: Some(2.0),
            ..Default::default()
        };

        assert!(opts.validate().is_ok());
    }

    #[test]
    fn valid_partial_options_pass() {
        let opts = ResizeOptions {
            width: Some(500),
            height: None,
            dpr: None,
            ..Default::default()
        };

        assert!(opts.validate().is_ok());
    }

    #[test]
    fn invalid_width_zero_fails() {
        let opts = ResizeOptions {
            width: Some(0),
            height: Some(100),
            dpr: None,
            ..Default::default()
        };

        let result = opts.validate();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Resize width must be greater than 0"
        );
    }

    #[test]
    fn invalid_height_zero_fails() {
        let opts = ResizeOptions {
            width: None,
            height: Some(0),
            dpr: None,
            ..Default::default()
        };

        let result = opts.validate();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Resize height must be greater than 0"
        );
    }

    #[test]
    fn invalid_dpr_low_fails() {
        let opts = ResizeOptions {
            width: Some(100),
            height: Some(100),
            dpr: Some(0.05),
            ..Default::default()
        };

        let result = opts.validate();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Resize dpr must be between 0.1 and 10.0"
        );
    }

    #[test]
    fn invalid_dpr_high_fails() {
        let opts = ResizeOptions {
            width: Some(100),
            height: Some(100),
            dpr: Some(42.0),
            ..Default::default()
        };

        let result = opts.validate();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Resize dpr must be between 0.1 and 10.0"
        );
    }
}
