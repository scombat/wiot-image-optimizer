use clap::Args;
use wiot_core::models::options::QualityOptions;

#[derive(Args, Debug)]
pub struct QualityArgs {
    /// Image quality (1-100, default: 80)
    #[arg(short, long)]
    quality: Option<u8>,
}

impl QualityArgs {
    pub fn get(&self) -> Option<QualityOptions> {
        match self.quality {
            Some(q) => QualityOptions::new(Some(q as f32)).ok(),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_args_get_none() {
        let args = QualityArgs { quality: None };
        let opts = args.get();
        assert!(opts.is_none());
    }

    #[test]
    fn test_quality_args_get_valid() {
        let args = QualityArgs { quality: Some(80) };
        let opts = args.get();
        assert!(opts.is_some());
        let opts = opts.unwrap();
        assert_eq!(opts.quality, Some(80.0));
    }

    #[test]
    fn test_quality_args_get_min() {
        let args = QualityArgs { quality: Some(1) };
        let opts = args.get();
        assert!(opts.is_some());
        let opts = opts.unwrap();
        assert_eq!(opts.quality, Some(1.0));
    }

    #[test]
    fn test_quality_args_get_max() {
        let args = QualityArgs { quality: Some(100) };
        let opts = args.get();
        assert!(opts.is_some());
        let opts = opts.unwrap();
        assert_eq!(opts.quality, Some(100.0));
    }
}
