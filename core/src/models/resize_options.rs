use anyhow::{Result, anyhow};
use clap::ValueEnum;
use image::imageops::FilterType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum AspectRatioStrategy {
    Fit,     // Default: proportional scaling (fit within width/height)
    Cover,   // Fill entire target size, possibly cropping
    Contain, // Fit inside target size without cropping, may leave empty space
    Stretch, // Force image to match exact dimensions (ignore aspect ratio)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResizeOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub dpr: f32,
    pub strategy: AspectRatioStrategy,
    pub filter: FilterType,
}

impl Default for ResizeOptions {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            dpr: 1.0,
            strategy: AspectRatioStrategy::Fit,
            filter: FilterType::Triangle, // Default to Triangle for higher-quality resizing
        }
    }
}

impl ResizeOptions {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        ResizeOptions {
            width,
            height,
            ..Self::default()
        }
    }

    pub fn dpr(mut self, dpr: f32) -> Self {
        self.dpr = dpr;
        self
    }

    pub fn strategy(mut self, strategy: AspectRatioStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn is_enabled(&self) -> bool {
        let is_size_modified = self.width.is_some() || self.height.is_some();
        let is_dpr_modified = self.dpr != 1.0;
        is_size_modified || is_dpr_modified
    }

    /*
     * Validate the resize options.
     *
     * - If width is Some, it must be > 0
     * - If height is Some, it must be > 0
     * - width and height can both be None or both be > 0
     * - DPR must be between 0.1 and 10.0 (default = 1.0)
     */
    #[allow(clippy::collapsible_if)]
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

        let dpr = self.dpr;
        if !(0.1..=10.0).contains(&dpr) {
            return Err(anyhow!(
                "Resize dpr must be between 0.1 and 10.0 (default is 1.0)"
            ));
        }

        if !self.is_enabled() {
            return Ok(());
        }

        match self.strategy {
            AspectRatioStrategy::Fit => {
                return Ok(());
            }
            _ => {
                if self.width.is_none() || self.height.is_none() {
                    return Err(anyhow!(
                        "{:?} strategy requires both width and height to be set",
                        self.strategy
                    ));
                }
            }
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
        assert_eq!(opts.dpr, 1.0);
    }

    mod is_enabled {
        use super::*;

        #[test]
        fn test_is_enabled() {
            // Full
            let opts = ResizeOptions::new(Some(800), Some(600));
            assert!(opts.is_enabled());

            // None
            let opts = ResizeOptions::new(None, None);
            assert!(!opts.is_enabled());

            // Width only
            let opts = ResizeOptions::new(Some(800), None);
            assert!(opts.is_enabled());

            // Height only
            let opts = ResizeOptions::new(None, Some(600));
            assert!(opts.is_enabled());

            // DPR only != 1.0
            let opts = ResizeOptions::new(None, None).dpr(2.0);
            assert!(opts.is_enabled());
        }
    }

    mod validate {
        use super::*;

        #[test]
        fn valid_resize_options_pass() {
            let opts = [
                ResizeOptions::new(Some(800), Some(600)),
                ResizeOptions::new(None, None).dpr(2.0),
                ResizeOptions::new(Some(800), None),
                ResizeOptions::new(None, Some(600)),
            ];
            for (i, opt) in opts.iter().enumerate() {
                assert!(
                    opt.validate().is_ok(),
                    "Failed to validate opts[{}] = {:?}",
                    i,
                    opt
                );
            }
        }

        #[test]
        fn invalid_resize_sizes_options_fails() {
            let opts = vec![
                ResizeOptions::new(Some(0), Some(100)),
                ResizeOptions::new(Some(0), None),
                ResizeOptions::new(None, Some(0)),
                ResizeOptions::new(Some(100), Some(0)),
            ];
            for opt in opts {
                assert!(opt.validate().is_err());
            }
        }

        #[test]
        fn invalid_dpr_fails() {
            let opts = vec![
                ResizeOptions::new(Some(100), Some(100)).dpr(0.0),
                ResizeOptions::new(Some(100), Some(100)).dpr(11.0),
                ResizeOptions::new(Some(100), Some(100)).dpr(0.05),
                ResizeOptions::new(Some(100), Some(100)).dpr(42.0),
            ];

            for opt in opts {
                assert!(opt.validate().is_err());
                assert_eq!(
                    opt.validate().unwrap_err().to_string(),
                    "Resize dpr must be between 0.1 and 10.0 (default is 1.0)"
                );
            }
        }

        mod aspect_ratio_strategy {
            use super::*;

            mod fit {
                use super::*;

                #[test]
                fn valid_resize_aspect_ratio_fit_strategy() {
                    let cases = vec![
                        ResizeOptions::new(Some(800), None),
                        ResizeOptions::new(None, Some(600)),
                        ResizeOptions::new(Some(1024), Some(768)),
                        ResizeOptions::new(Some(500), None).dpr(1.5),
                        ResizeOptions::new(None, Some(500)).dpr(0.5),
                        ResizeOptions::new(Some(1200), Some(800)).dpr(2.0),
                        ResizeOptions::new(None, None).dpr(1.2),
                        ResizeOptions::new(None, None),
                        ResizeOptions::new(Some(300), Some(200)).strategy(AspectRatioStrategy::Fit),
                    ];

                    for (i, opts) in cases.into_iter().enumerate() {
                        assert!(
                            opts.validate().is_ok(),
                            "Fit strategy should accept case #{}: {:?}",
                            i,
                            opts
                        );
                    }
                }

                #[test]
                fn invalid_resize_aspect_ratio_fit_strategy() {
                    let cases = vec![
                        ResizeOptions::new(Some(0), None).dpr(1.0),
                        ResizeOptions::new(None, Some(0)).dpr(1.0),
                        ResizeOptions::new(None, None).dpr(0.05),
                        ResizeOptions::new(None, None).dpr(20.0),
                        ResizeOptions::new(Some(800), None).dpr(0.05),
                        ResizeOptions::new(None, Some(600)).dpr(20.0),
                    ];

                    for (i, opts) in cases.into_iter().enumerate() {
                        assert!(
                            opts.validate().is_err(),
                            "Fit strategy should reject case #{}: {:?}",
                            i,
                            opts
                        );
                    }
                }
            }

            mod others {
                use crate::models::resize_options;

                use super::*;
                static STRATEGIES: &[resize_options::AspectRatioStrategy] = &[
                    AspectRatioStrategy::Cover,
                    AspectRatioStrategy::Contain,
                    AspectRatioStrategy::Stretch,
                ];
                #[test]
                fn with_width_and_height_defined_should_be_valid_and_enabled() {
                    let cases = [
                        Some(1024),
                        Some(768),
                        Some(1200),
                        Some(800),
                        Some(300),
                        Some(200),
                    ];

                    for (i, opts) in cases.chunks(2).enumerate() {
                        for strategy in STRATEGIES.iter() {
                            let operation =
                                ResizeOptions::new(opts[0], opts[1]).strategy(*strategy);
                            assert!(
                                operation.validate().is_ok() && operation.is_enabled(),
                                "{:?} strategy should accept case #{}: {:?}",
                                strategy,
                                i,
                                opts
                            );
                        }
                    }
                }

                #[test]
                fn without_width_or_height_should_be_invalid_or_disabled() {
                    let cases = [
                        None,
                        Some(768),
                        Some(1200),
                        None,
                        None,
                        None,
                        Some(0),
                        Some(768),
                    ];

                    for (i, opts) in cases.chunks(2).enumerate() {
                        for strategy in STRATEGIES.iter() {
                            let operation =
                                ResizeOptions::new(opts[0], opts[1]).strategy(*strategy);
                            assert!(
                                operation.validate().is_err() || !operation.is_enabled(),
                                "{:?} strategy should not accept case #{}: {:?}",
                                strategy,
                                i,
                                opts
                            );
                        }
                    }
                }
            }
        }
    }
}
