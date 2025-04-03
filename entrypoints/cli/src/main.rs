use adapters::resolver::AdapterResolver;
use anyhow::Result;
use clap::Parser;
use wiot_core::ImagePipeline;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    // Use automatic adapter resolution
    // This will resolve the source and destination based on the input and output paths
    // For example, if the input is "file:///path/to/image.jpg", it will use the LocalFileAdapter
    // This can be bypass and overridden by the user if they want to use a specific adapter
    let source = AdapterResolver::resolve_source(&args.input)?;
    let destination = AdapterResolver::resolve_destination(&args.output)?;

    // Create a new ImagePipeline instance
    let mut pipeline = ImagePipeline::new(source, destination, &args.input, &args.output);

    // Run the pipeline
    pipeline.run().await?;

    Ok(())
}
