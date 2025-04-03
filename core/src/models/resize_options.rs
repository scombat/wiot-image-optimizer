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
            dpr: Some(1.0),
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
        self.width.is_some()
            || self.height.is_some()
            || (self.dpr.is_some() && self.dpr != Some(1.0))
    }

    /*
     * Validate the resize options.
     *
     * - If width is Some, it must be > 0
     * - If height is Some, it must be > 0
     * - width and height can both be None or both be > 0
     * - DPR must be between 0.1 and 10.0 (default = 1.0)
     */
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if let Some(width) = self.width {
            if width == 0 {
                return Err(anyhow!("Resize width must be greater than 0"));
            }
        }

        if let Some(height) = self.height {
            if height == 0 {
                return Err(anyhow!("Resize height must be greater than 0"));
            }
        }

        let dpr = self.dpr.unwrap_or(1.0);
        if !(0.1..=10.0).contains(&dpr) {
            return Err(anyhow!(
                "Resize dpr must be between 0.1 and 10.0 (default is 1.0)"
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dpr() {
        let opts = ResizeOptions::default();
        assert_eq!(opts.dpr, Some(1.0));
    }

    mod is_enabled {
        use super::*;

        #[test]
        fn test_is_enabled() {
            // Full
            let opts = ResizeOptions::new(Some(800), Some(600), Some(2.0)).unwrap();
            assert!(opts.is_enabled());

            // None
            let opts = ResizeOptions::new(None, None, None).unwrap();
            assert!(!opts.is_enabled());

            // Width only
            let opts = ResizeOptions::new(Some(800), None, None).unwrap();
            assert!(opts.is_enabled());

            // Height only
            let opts = ResizeOptions::new(None, Some(600), None).unwrap();
            assert!(opts.is_enabled());

            // DPR only != 1.0
            let opts = ResizeOptions::new(None, None, Some(2.0)).unwrap();
            assert!(opts.is_enabled());
        }
    }

    mod validate {
        use super::*;

        #[test]
        fn valid_resize_options_pass() {
            let opts = vec![
                ResizeOptions::new(Some(800), Some(600), Some(2.0)),
                ResizeOptions::new(None, None, None),
                ResizeOptions::new(Some(800), None, Some(2.0)),
                ResizeOptions::new(None, Some(600), Some(2.0)),
                ResizeOptions::new(None, None, Some(2.0)),
            ];
            for opt in opts {
                assert!(opt.is_ok());
            }
        }

        #[test]
        fn invalid_resize_sizes_options_fails() {
            let opts = vec![
                ResizeOptions::new(Some(0), Some(100), None),
                ResizeOptions::new(Some(0), None, Some(2.0)),
                ResizeOptions::new(None, Some(0), Some(2.0)),
                ResizeOptions::new(Some(100), Some(0), None),
            ];
            for opt in opts {
                assert!(opt.is_err());
            }
        }

        #[test]
        fn invalid_dpr_fails() {
            let opts = vec![
                ResizeOptions::new(Some(100), Some(100), Some(0.0)),
                ResizeOptions::new(Some(100), Some(100), Some(11.0)),
                ResizeOptions::new(Some(100), Some(100), Some(0.05)),
                ResizeOptions::new(Some(100), Some(100), Some(42.0)),
            ];

            for opt in opts {
                assert!(opt.is_err());
                assert_eq!(
                    opt.unwrap_err().to_string(),
                    "Resize dpr must be between 0.1 and 10.0 (default is 1.0)"
                );
            }
        }
    }
}
