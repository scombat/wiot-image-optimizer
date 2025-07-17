use clap::Args;
use wiot_core::models::options::RotateOptions;

#[derive(Args, Debug)]
pub struct RotateArgs {
    // Rotate image
    #[arg(long, short)]
    pub rotate: Option<f32>,
}

impl RotateArgs {
    pub fn get(&self) -> Option<RotateOptions> {
        self.rotate?;

        Some(RotateOptions {
            angle_degrees: self.rotate,
        })
    }
}
