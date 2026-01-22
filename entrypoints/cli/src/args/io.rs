use std::sync::Arc;

use clap::Args;
use wiot_core::services::io::{FileDestination, FileSource};

use adapters::resolver::AdapterResolver;

#[derive(Args, Debug)]
pub struct IoArgs {
    /// Input image path
    #[arg(short, long)]
    pub input: String,

    /// Output image path
    #[arg(short, long)]
    pub output: Option<String>,
}

impl IoArgs {
    pub fn get_output(&self) -> String {
        match &self.output {
            Some(s) => s.to_string(),
            None => "./".to_string(),
        }
    }

    pub fn resolve_source(&self) -> Result<Arc<dyn FileSource + 'static>, anyhow::Error> {
        AdapterResolver::resolve_source(&self.input)
    }

    pub fn resolve_destination(&self) -> Result<Arc<dyn FileDestination>, anyhow::Error> {
        AdapterResolver::resolve_destination(&self.get_output())
    }
}
