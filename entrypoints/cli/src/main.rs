use adapters::resolver::AdapterResolver;
use anyhow::Result;
use clap::Parser;
use wiot_core::models::quality_options::QualityOptions;
use wiot_core::{ImagePipeline, models::options::ProcessingOptions};

#[derive(Parser, Debug)]
#[command(name = "wiot-cli")]
#[command(about = "Run the WIOT image processor from CLI", long_about = None)]
struct CliArgs {
    /// Input image path
    #[arg(short, long)]
    input: String,

    /// Output image path
    #[arg(short, long)]
    output: String,

    /// Image quality (1-100, default: 80)
    #[arg(short, long)]
    quality: Option<u8>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    // Use automatic adapter resolution
    let source = AdapterResolver::resolve_source(&args.input)?;
    let destination = AdapterResolver::resolve_destination(&args.output)?;

    // Create quality options if specified
    let quality_options = if args.quality.is_some() {
        Some(QualityOptions::new(Some(args.quality.unwrap() as f32))?)
    } else {
        None
    };

    // Create processing options
    let options = ProcessingOptions {
        quality: quality_options,
        ..Default::default()
    };

    // Create a new ImagePipeline instance with options
    let mut pipeline =
        ImagePipeline::with_options(source, destination, &args.input, &args.output, options);

    // Run the pipeline
    pipeline.run().await?;

    Ok(())
}
