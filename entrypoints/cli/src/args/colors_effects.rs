use clap::Args;
use wiot_core::models::options::ColorsOptions;

#[derive(Args, Debug)]
pub struct ColorsEffectsArgs {
    /// Convert image to grayscale
    #[arg(long)]
    pub grayscale: bool,

    /// Adjust brightness (can be negative)
    #[arg(long, allow_hyphen_values = true)]
    pub brightness: Option<i32>,

    /// Adjust contrast (factor, e.g. 1.2 = +20%)
    #[arg(long, allow_hyphen_values = true)]
    pub contrast: Option<f32>,

    /// Apply gamma correction (exponent)
    #[arg(long)]
    pub gamma: Option<f32>,

    /// Invert colors (negative effect)
    #[arg(long)]
    pub invert: bool,
}

impl ColorsEffectsArgs {
    pub fn get(&self) -> Option<ColorsOptions> {
        Some(ColorsOptions {
            grayscale: self.grayscale,
            brightness: self.brightness,
            contrast: self.contrast,
            gamma: self.gamma,
            invert: self.invert,
        })
    }
}
