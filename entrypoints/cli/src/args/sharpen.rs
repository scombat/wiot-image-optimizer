use clap::{ArgGroup, Args};
use wiot_core::models::image_adjustments_options::Sharpen;

#[derive(Args, Debug)]
#[command(
    group(
        ArgGroup::new("sharpen")
            .args(&["sharpen_sigma", "sharpen_threshold"])
            .required(false)
            .multiple(true)
    )
)]
pub struct SharpenArgs {
    /// Amount to blur the image by before inverting (unsharp mask)
    #[clap(long, help_heading = "SHARPEN OPTIONS")]
    pub sharpen_sigma: Option<f32>,

    /// How strong the sharpening should be (threshold)
    #[clap(long, help_heading = "SHARPEN OPTIONS")]
    pub sharpen_threshold: Option<i32>,
}

impl SharpenArgs {
    pub fn get(&self) -> Option<Sharpen> {
        if let (Some(sigma), Some(threshold)) = (self.sharpen_sigma, self.sharpen_threshold) {
            return Some(Sharpen { sigma, threshold });
        }
        None
    }
}
