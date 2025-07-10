pub mod models;
mod pipeline;
pub mod services;
pub mod utils;

use image::DynamicImage;
use models::options::ProcessingOptions;
use services::{
    encoding::codec_resolver::CodecResolver,
    io::{FileDestination, FileSource},
};
use std::sync::Arc;

pub struct ImagePipeline<'a> {
    source: Arc<dyn FileSource>,
    destination: Arc<dyn FileDestination>,
    input: &'a str,
    output: String,
    options: ProcessingOptions,
    image: Option<DynamicImage>,
    codec_resolver: CodecResolver,
    encoded_bytes: Option<Vec<u8>>,
}

impl<'a> ImagePipeline<'a> {
    pub fn new(
        source: Arc<dyn FileSource>,
        destination: Arc<dyn FileDestination>,
        input: &'a str,
        output: String,
    ) -> Self {
        ImagePipeline {
            source,
            destination,
            input,
            output,
            options: ProcessingOptions::default(),
            image: None,
            codec_resolver: CodecResolver::default(),
            encoded_bytes: None,
        }
    }

    pub fn with_options(
        source: Arc<dyn FileSource>,
        destination: Arc<dyn FileDestination>,
        input: &'a str,
        output: String,
        options: ProcessingOptions,
    ) -> Self {
        ImagePipeline {
            source,
            destination,
            input,
            output,
            options,
            image: None,
            codec_resolver: CodecResolver::default(),
            encoded_bytes: None,
        }
    }

    pub fn set_options(&mut self, options: ProcessingOptions) {
        self.options = options;
    }
}
