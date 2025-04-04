pub mod models;
pub mod services;

use image::DynamicImage;
use models::options::ProcessingOptions;
use services::{io::FileDestination, io::FileSource, encoder::{get_encoder, ImageEncoder}};
use std::sync::Arc;

pub struct ImagePipeline<'a> {
    source: Arc<dyn FileSource>,
    destination: Arc<dyn FileDestination>,
    input: &'a str,
    output: &'a str,
    options: ProcessingOptions,
    image: Option<DynamicImage>,
}

impl<'a> ImagePipeline<'a> {
    pub fn new(
        source: Arc<dyn FileSource>,
        destination: Arc<dyn FileDestination>,
        input: &'a str,
        output: &'a str,
    ) -> Self {
        ImagePipeline {
            source,
            destination,
            input,
            output,
            options: ProcessingOptions::default(),
            image: None,
        }
    }

    pub async fn load(&mut self) -> Result<(), anyhow::Error> {
        self.image = Some(self.source.read(self.input).await?);
        if self.image.is_none() {
            return Err(anyhow::anyhow!("[core/pipeline] Image cannot be loaded"));
        }
        Ok(())
    }

    pub async fn store(&mut self) -> Result<(), anyhow::Error> {
        let image = self.image
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("[core/pipeline] Image cannot be loaded"))?;

        // Determine output format from options or file extension
        let format = self.options.quality.as_ref()
            .and_then(|q| q.format.as_deref())
            .unwrap_or_else(|| {
                // Extract format from output path extension
                self.output.split('.')
                    .last()
                    .unwrap_or("jpeg")
            });

        // Get encoder for the format
        let encoder = get_encoder(format)
            .ok_or_else(|| anyhow::anyhow!("[core/pipeline] Unsupported output format: {}", format))?;

        // Encode the image
        let encoded = encoder.encode(image, &self.options)?;

        // Write the encoded bytes
        self.destination.write(self.output, &encoded, encoder.format()).await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        self.load().await?;
        self.process_image()?;
        self.store().await?;
        Ok(())
    }

    fn process_image(&mut self) -> Result<(), anyhow::Error> {
        self.resize()?;
        self.optimize_quality()?;
        Ok(())
    }
}
