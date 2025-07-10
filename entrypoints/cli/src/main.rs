use adapters::resolver::AdapterResolver;
use anyhow::Result;
use clap::Parser;
use wiot_core::models::quality_options::QualityOptions;
use wiot_core::models::resize_options::{AspectRatioStrategy, ResizeOptions};
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
    output: Option<String>,

    /// Image quality (1-100, default: 80)
    #[arg(short, long)]
    quality: Option<u8>,

    /// Width of the output image.
    ///
    /// If not specified, the image width will remain unchanged.
    /// If only one of width or height is specified, the other dimension will be automatically adjusted
    /// to preserve the original aspect ratio, unless the --aspect-ratio option is set to override this behavior.
    #[arg(short = 'W', long)]
    width: Option<u32>,

    /// Height of the output image.
    ///
    /// If not specified, the image height will remain unchanged.
    /// If only one of width or height is specified, the other dimension will be automatically adjusted
    /// to preserve the original aspect ratio, unless the --aspect-ratio option is set to override this behavior.
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// Device Pixel Ratio (DPR) multiplier for output image dimensions (0.1 - 10.0).
    ///
    /// If specified, the output image's width and height will be multiplied by this value.
    /// For example, a DPR of 2.0 will double the output dimensions, which is useful for generating
    /// high-resolution images for retina or high-DPI displays.
    /// If not specified, the DPR defaults to 1.0 (no scaling).
    #[arg(long)]
    dpr: Option<f32>,

    /// Aspect ratio strategy (Fit, Cover, Contain, Stretch, default: Fit)
    #[arg(short, long, value_enum)]
    aspect_ratio: Option<AspectRatioStrategy>,

    /// Output image format (jpeg, png, webp, avif)
    ///
    /// If not specified, the format will be inferred from the output file extension,
    /// unless --auto-format is enabled.
    #[arg(long, value_name = "FORMAT", conflicts_with = "auto_format")]
    format: Option<String>,

    /// Enable automatic selection of the output format.
    ///
    /// The format will be chosen to produce the smallest image.
    /// Default formats are: WebP, Avif, and Jpeg or Png depending on whether the image contains transparency.
    /// This flag is incompatible with the "format" option.
    #[arg(long, conflicts_with_all = &["format"])]
    auto_format: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    // Use automatic adapter resolution
    let output = args.output.clone().unwrap_or_else(|| "./".to_string());
    let source = AdapterResolver::resolve_source(&args.input)?;
    let destination = AdapterResolver::resolve_destination(&output)?;

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
        auto_select_format: args.auto_format,
        ..Default::default()
    };

    // Create a new ImagePipeline instance with options
    let mut pipeline =
        ImagePipeline::with_options(source, destination, &args.input, output, options);

    // Run the pipeline
    pipeline.run().await?;

    Ok(())
}
