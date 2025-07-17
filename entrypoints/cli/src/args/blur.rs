use clap::Args;
use wiot_core::models::options::BlurOptions;

#[derive(Args, Debug)]
pub struct BlurArgs {
    // Blur (Standard deviation of the Gaussian kernel, in pixels)
    #[arg(short, long)]
    pub blur: Option<f32>,
}

impl BlurArgs {
    pub fn get(&self) -> Option<BlurOptions> {
        self.blur.map(|v| BlurOptions { blur: Some(v) })
    }
}
