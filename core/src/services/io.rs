use anyhow::Result;
use async_trait::async_trait;
use image::DynamicImage;
use std::any::Any;
use std::sync::Arc;

#[async_trait]
pub trait FileSource: Any + Send + Sync {
    async fn read(&self, path: &str) -> Result<DynamicImage>;
    fn as_any(&self) -> &dyn Any;
}

#[async_trait]
pub trait FileDestination: Any + Send + Sync {
    async fn write(&self, path: &str, data: &[u8]) -> Result<()>;
    fn as_any(&self) -> &dyn Any;
}

pub trait FileAdapterFactory: Any + Send + Sync {
    fn can_handler(&self, uri: &str) -> bool;
    fn create_source(&self, uri: &str) -> Result<Arc<dyn FileSource>>;
    fn create_destination(&self, uri: &str) -> Result<Arc<dyn FileDestination>>;
    fn as_any(&self) -> &dyn Any;
}
