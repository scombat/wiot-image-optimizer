use anyhow::Result;
use clap::Parser;
use wiot_core::models::options::GravityOptions;
use wiot_core::{ImagePipeline, models::options::ProcessingOptions};

mod args;
use crate::args::{
    background::BackgroundArgs, blur::BlurArgs, crop::CropArgs, format::FormatArgs,
    image_adjustments::ImageAdjustmentsArgs, io::IoArgs, mirror::MirrorArgs, quality::QualityArgs,
    resize::ResizeArgs, rotate::RotateArgs,
};

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

    #[arg(short, long, value_enum)]
    pub gravity: Option<GravityOptions>,

    #[command(flatten)]
    pub mirror: MirrorArgs,

    #[command(flatten)]
    pub rotate: RotateArgs,

    #[command(flatten)]
    pub background: BackgroundArgs,

    #[command(flatten)]
    pub blur: BlurArgs,

    #[command(flatten)]
    pub image_adjustments: ImageAdjustmentsArgs,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    let input = args.io.input.as_str();
    let output = args.io.get_output();
    let source = args.io.resolve_source()?;
    let destination = args.io.resolve_destination()?;
    let options = ProcessingOptions {
        quality: args.quality.get(),
        resize: args.resize.get(),
        crop: args.crop.get(),
        auto_select_format: args.format.auto_format,
        gravity: args.gravity,
        mirror: args.mirror.get(),
        rotate: args.rotate.get(),
        blur: args.blur.get(),
        background: args.background.get(),
        image_adjustments: args.image_adjustments.get(),
        ..Default::default()
    };

    // Create a new ImagePipeline instance with options
    let mut pipeline = ImagePipeline::with_options(source, destination, input, output, options);

    // Run the pipeline
    pipeline.run().await?;

    Ok(())
}
