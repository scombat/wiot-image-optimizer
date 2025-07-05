use adapters::resolver::AdapterResolver;
use anyhow::Result;
use clap::Parser;
use wiot_core::models::options::ResizeOptions;
use wiot_core::models::quality_options::QualityOptions;
use wiot_core::models::resize_options::AspectRatioStrategy;
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

    /// Width
    #[arg(short = 'W', long)]
    width: Option<u32>,

    /// Height
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// DPR
    #[arg(long)]
    dpr: Option<f32>,

    /// Aspect ratio strategy (Fit, Cover, Contain, Stretch, default: Fit)
    #[arg(short, long, value_enum)]
    aspect_ratio: Option<AspectRatioStrategy>,
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

    // Resize options
    let resize_options = if args.width.is_some()
        || args.height.is_some()
        || args.dpr.is_some()
        || args.aspect_ratio.is_some()
    {
        let mut ro = ResizeOptions::new(args.width, args.height);

        if let Some(dpr_val) = args.dpr {
            ro = ro.dpr(dpr_val);
        }
        if let Some(strat) = args.aspect_ratio {
            ro = ro.strategy(strat);
        }

        Some(ro)
    } else {
        None
    };

    // Create processing options
    let options = ProcessingOptions {
        quality: quality_options,
        resize: resize_options,
        ..Default::default()
    };

    // Create a new ImagePipeline instance with options
    let mut pipeline =
        ImagePipeline::with_options(source, destination, &args.input, &args.output, options);

    // Run the pipeline
    pipeline.run().await?;

    Ok(())
}
