pub mod services;

use std::sync::Arc;

use image::DynamicImage;
use services::{io::FileDestination, io::FileSource};

pub struct ImagePipeline<'a> {
    source: Arc<dyn FileSource>,
    destination: Arc<dyn FileDestination>,
    input: &'a str,
    output: &'a str,
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
        }
    }

    pub async fn run(&mut self) {
        let image = match self.source.read(self.input).await {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Error reading image: {}", e);
                return;
            }
        };

        let processed_image = match self.process_image(image) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Error processing image: {}", e);
                return;
            }
        };

        if let Err(e) = self.destination.write(self.output, &processed_image).await {
            eprintln!("Error writing image: {}", e);
        }
    }

    fn process_image(&self, image: DynamicImage) -> Result<DynamicImage, String> {
        // Placeholder for image processing logic
        Ok(image)
    }
}
