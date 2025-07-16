use clap::Args;
use wiot_core::models::resize_options::{AspectRatioStrategy, ResizeOptions};

#[derive(Args, Debug)]
pub struct ResizeArgs {
    /// Width of the output image.
    ///
    /// If not specified, the image width will remain unchanged.
    /// If only one of width or height is specified, the other dimension will be automatically adjusted
    /// to preserve the original aspect ratio, unless the --aspect-ratio option is set to override this behavior.
    #[arg(short = 'W', long)]
    pub width: Option<u32>,

    /// Height of the output image.
    ///
    /// If not specified, the image height will remain unchanged.
    /// If only one of width or height is specified, the other dimension will be automatically adjusted
    /// to preserve the original aspect ratio, unless the --aspect-ratio option is set to override this behavior.
    #[arg(short = 'H', long)]
    pub height: Option<u32>,

    /// Device Pixel Ratio (DPR) multiplier for output image dimensions (0.1 - 10.0).
    ///
    /// If specified, the output image's width and height will be multiplied by this value.
    /// For example, a DPR of 2.0 will double the output dimensions, which is useful for generating
    /// high-resolution images for retina or high-DPI displays.
    /// If not specified, the DPR defaults to 1.0 (no scaling).
    #[arg(long)]
    pub dpr: Option<f32>,

    /// Aspect ratio strategy (Fit, Cover, Contain, Stretch, default: Fit)
    #[arg(short, long, value_enum)]
    pub aspect_ratio: Option<AspectRatioStrategy>,
}

impl ResizeArgs {
    pub fn get(&self) -> Option<ResizeOptions> {
        Some(
            ResizeOptions::new(self.width, self.height)
                .dpr(self.dpr.unwrap_or(1.0))
                .strategy(self.aspect_ratio.unwrap_or(AspectRatioStrategy::Fit)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiot_core::models::resize_options::AspectRatioStrategy;

    #[test]
    fn test_resize_args_get_all_none() {
        let args = ResizeArgs {
            width: None,
            height: None,
            dpr: None,
            aspect_ratio: None,
        };
        let opts = args.get().unwrap();
        assert_eq!(opts.width, None);
        assert_eq!(opts.height, None);
        assert_eq!(opts.dpr, 1.0);
        assert_eq!(opts.strategy, AspectRatioStrategy::Fit);
    }

    #[test]
    fn test_resize_args_get_width_only() {
        let args = ResizeArgs {
            width: Some(800),
            height: None,
            dpr: None,
            aspect_ratio: None,
        };
        let opts = args.get().unwrap();
        assert_eq!(opts.width, Some(800));
        assert_eq!(opts.height, None);
        assert_eq!(opts.dpr, 1.0);
        assert_eq!(opts.strategy, AspectRatioStrategy::Fit);
    }

    #[test]
    fn test_resize_args_get_height_only() {
        let args = ResizeArgs {
            width: None,
            height: Some(600),
            dpr: None,
            aspect_ratio: None,
        };
        let opts = args.get().unwrap();
        assert_eq!(opts.width, None);
        assert_eq!(opts.height, Some(600));
        assert_eq!(opts.dpr, 1.0);
        assert_eq!(opts.strategy, AspectRatioStrategy::Fit);
    }

    #[test]
    fn test_resize_args_get_width_height() {
        let args = ResizeArgs {
            width: Some(1024),
            height: Some(768),
            dpr: None,
            aspect_ratio: None,
        };
        let opts = args.get().unwrap();
        assert_eq!(opts.width, Some(1024));
        assert_eq!(opts.height, Some(768));
        assert_eq!(opts.dpr, 1.0);
        assert_eq!(opts.strategy, AspectRatioStrategy::Fit);
    }

    #[test]
    fn test_resize_args_get_with_dpr() {
        let args = ResizeArgs {
            width: Some(500),
            height: Some(400),
            dpr: Some(2.0),
            aspect_ratio: None,
        };
        let opts = args.get().unwrap();
        assert_eq!(opts.width, Some(500));
        assert_eq!(opts.height, Some(400));
        assert_eq!(opts.dpr, 2.0);
        assert_eq!(opts.strategy, AspectRatioStrategy::Fit);
    }

    #[test]
    fn test_resize_args_get_with_aspect_ratio() {
        let args = ResizeArgs {
            width: Some(300),
            height: Some(200),
            dpr: None,
            aspect_ratio: Some(AspectRatioStrategy::Cover),
        };
        let opts = args.get().unwrap();
        assert_eq!(opts.width, Some(300));
        assert_eq!(opts.height, Some(200));
        assert_eq!(opts.dpr, 1.0);
        assert_eq!(opts.strategy, AspectRatioStrategy::Cover);
    }

    #[test]
    fn test_resize_args_get_with_all_options() {
        let args = ResizeArgs {
            width: Some(1920),
            height: Some(1080),
            dpr: Some(1.5),
            aspect_ratio: Some(AspectRatioStrategy::Stretch),
        };
        let opts = args.get().unwrap();
        assert_eq!(opts.width, Some(1920));
        assert_eq!(opts.height, Some(1080));
        assert_eq!(opts.dpr, 1.5);
        assert_eq!(opts.strategy, AspectRatioStrategy::Stretch);
    }

    #[test]
    fn test_resize_args_get_with_zero_dpr() {
        let args = ResizeArgs {
            width: Some(100),
            height: Some(100),
            dpr: Some(0.0),
            aspect_ratio: None,
        };
        let opts = args.get().unwrap();
        assert_eq!(opts.dpr, 0.0);
    }

    #[test]
    fn test_resize_args_get_with_negative_dpr() {
        let args = ResizeArgs {
            width: Some(100),
            height: Some(100),
            dpr: Some(-2.0),
            aspect_ratio: None,
        };
        let opts = args.get().unwrap();
        assert_eq!(opts.dpr, -2.0);
    }
}
