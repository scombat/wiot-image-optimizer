pub mod models;
pub mod services;

use image::DynamicImage;
use models::options::ProcessingOptions;
use services::{io::FileDestination, io::FileSource};
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
        self.destination
            .write(
                self.output,
                self.image
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("[core/pipeline] Image cannot be loaded"))?,
            )
            .await?;
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
        Ok(())
    }
}
