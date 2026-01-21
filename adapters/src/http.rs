use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use image::DynamicImage;
use reqwest::Client;
use wiot_core::services::io::{FileAdapterFactory, FileDestination, FileSource};

pub struct HttpSource {
    client: Client,
}

#[async_trait]
impl FileSource for HttpSource {
    async fn read(&self, path: &str) -> Result<DynamicImage> {
        let bytes = self.client.get(path).send().await?.bytes().await?;
        let img = image::load_from_memory(&bytes[..])?;
        Ok(img)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct HttpDestination {
    buffer: Arc<Mutex<Vec<u8>>>,
}

#[async_trait]
impl FileDestination for HttpDestination {
    async fn write(&self, _path: &str, data: &[u8]) -> Result<()> {
        let mut buf = self.buffer.lock().unwrap();
        buf.clear();
        buf.extend_from_slice(data);
        Ok(())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct HttpFileAdapterFactory;
impl FileAdapterFactory for HttpFileAdapterFactory {
    fn can_handle(&self, uri: &str) -> bool {
        uri.starts_with("http://") || uri.starts_with("https://")
    }

    fn create_source(&self, uri: &str) -> Result<Arc<dyn FileSource>> {
        if !self.can_handle(uri) {
            anyhow::bail!("HttpAdapter cannot handle URI: {}", uri);
        }
        Ok(Arc::new(HttpSource {
            client: Client::new(),
        }))
    }

    fn create_destination(&self, uri: &str) -> Result<Arc<dyn FileDestination>> {
        if !self.can_handle(uri) {
            anyhow::bail!("HttpAdapter cannot handle URI: {}", uri);
        }
        Ok(Arc::new(HttpDestination {
            buffer: Arc::new(Mutex::new(Vec::new())),
        }))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
