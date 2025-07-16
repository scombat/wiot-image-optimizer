use anyhow::Result;
use clap::Parser;
use wiot_core::{ImagePipeline, models::options::ProcessingOptions};
mod args;
use crate::args::{
    crop::CropArgs, format::FormatArgs, io::IoArgs, quality::QualityArgs, resize::ResizeArgs,
};
use wiot_core::models::options::GravityOptions;

#[derive(Parser, Debug)]
#[command(name = "wiot-cli")]
#[command(about = "Run the WIOT image processor from CLI", long_about = None)]
struct CliArgs {
    #[command(flatten)]
    pub io: IoArgs,

    #[command(flatten)]
    pub resize: ResizeArgs,

    #[command(flatten)]
    pub crop: CropArgs,

    #[command(flatten)]
    pub format: FormatArgs,

    #[command(flatten)]
    pub quality: QualityArgs,

    /// Anchor when no offsets are given
    #[arg(short, long, value_enum)]
    pub gravity: Option<GravityOptions>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    let input = args.io.input.as_str();
    let output = args.io.get_output();
    let source = args.io.resolve_source().unwrap();
    let destination = args.io.resolve_destination().unwrap();
    let options = ProcessingOptions {
        quality: args.quality.get(),
        resize: args.resize.get(),
        crop: args.crop.get(),
        auto_select_format: args.format.auto_format,
        gravity: args.gravity,
        ..Default::default()
    };

    // Create a new ImagePipeline instance with options
    let mut pipeline = ImagePipeline::with_options(source, destination, input, output, options);

    // Run the pipeline
    pipeline.run().await?;

    Ok(())
}
