use anyhow::Result;
use async_trait::async_trait;
use image::{DynamicImage, open};
use std::sync::Arc;
use tokio::task::spawn_blocking;
use wiot_core::services::io::{FileAdapterFactory, FileDestination, FileSource};

#[derive(Clone)]
pub struct LocalFileAdapter;

#[async_trait]
impl FileSource for LocalFileAdapter {
    async fn read(&self, path: &str) -> Result<DynamicImage> {
        let path = path.to_string();
        match spawn_blocking(move || open(&path)).await? {
            Ok(img) => Ok(img),
            Err(e) => Err(anyhow::anyhow!(
                "[LocalFileSource] Failed to read image: {}",
                e
            )),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[async_trait]
impl FileDestination for LocalFileAdapter {
    async fn write(&self, path: &str, image: &DynamicImage) -> Result<()> {
        let path = path.to_string();
        let img: DynamicImage = image.clone();
        match spawn_blocking(move || img.save(path)).await? {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!(
                "[LocalFileDestination] Failed to write image: {}",
                e
            )),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl FileAdapterFactory for LocalFileAdapter {
    fn can_handler(&self, uri: &str) -> bool {
        match uri.split_once("://") {
            Some((scheme, _)) => scheme == "file" || scheme.is_empty(),
            None => true,
        }
    }

    fn create_source(&self, uri: &str) -> Result<Arc<dyn FileSource>> {
        if self.can_handler(uri) {
            Ok(Arc::new(LocalFileAdapter))
        } else {
            Err(anyhow::anyhow!("Unsupported URI scheme"))
        }
    }

    fn create_destination(&self, uri: &str) -> Result<Arc<dyn FileDestination>> {
        if self.can_handler(uri) {
            Ok(Arc::new(LocalFileAdapter))
        } else {
            Err(anyhow::anyhow!("Unsupported URI scheme"))
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn image_path(name: &str) -> String {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../tests/assets/");
        path.push(name);
        path.to_str()
            .expect("Failed to convert PathBuf to str")
            .to_string()
    }
    fn dummy_image() -> DynamicImage {
        image::DynamicImage::new_rgb8(10, 10)
    }

    mod factory {
        use super::*;

        #[tokio::test]
        async fn test_factory() {
            let adapter = LocalFileAdapter;
            let uris = vec![
                "file://path/to/image.png",
                "image.png",
                "./img.jpeg",
                "../../img.webp",
                "C:\\\\Windows\\img.gif",
            ];

            for uri in uris {
                assert!(
                    adapter.can_handler(uri),
                    "Adapter should handle file scheme"
                );
                assert!(
                    adapter.create_source(uri).is_ok(),
                    "Source should be created"
                );
                assert!(
                    adapter.create_destination(uri).is_ok(),
                    "Destination should be created"
                );
            }

            let valid_img = image_path("test.png");
            assert!(
                Path::new(&valid_img).exists(),
                "Missing test image at tests/assets/test.png"
            );
            let result = adapter.read(&valid_img).await;
            assert!(result.is_ok(), "Failed to read image");
            let image = result.unwrap();
            assert!(
                image.width() == 512,
                "Expected a valid image: not a DynamicImage or missmatch width"
            );
            assert!(
                adapter.write(&valid_img, &image).await.is_ok(),
                "Failed to write image"
            );
            assert!(Path::new(&valid_img).exists(), "Output file not created");
        }

        #[tokio::test]
        async fn test_factory_failure() {
            let uris = vec![
                "http://path/to/image.png",
                "https://path/to/image.png",
                "s3://path/to/image.png",
            ];

            let adapter = LocalFileAdapter;
            for uri in uris {
                assert!(
                    !adapter.can_handler(uri),
                    "Adapter should not handle non-file scheme"
                );
                assert!(
                    adapter.create_source(uri).is_err(),
                    "Source creation should fail for non-file scheme"
                );
                assert!(
                    adapter.create_destination(uri).is_err(),
                    "Destination creation should fail for non-file scheme"
                );
            }
        }
    }

    mod read {
        use super::*;

        #[tokio::test]
        async fn test_read_image() {
            let path = image_path("test.png");
            assert!(
                std::path::Path::new(&path).exists(),
                "Missing test image at tests/assets/test.png"
            );

            let adapter = LocalFileAdapter;
            let result = adapter.read(&path).await;
            assert!(result.is_ok());

            let image = result.unwrap();
            assert!(
                image.width() == 512,
                "Expected a valid image: not a DynamicImage or missmatch width"
            );
        }

        #[tokio::test]
        async fn test_read_image_failure() {
            let path = image_path("no-exist.jpeg");
            assert!(
                !Path::new(&path).exists(),
                "Test image should not exist at tests/assets/no-exist.jpeg"
            );

            let adapter = LocalFileAdapter;
            let result = adapter.read(&path).await;
            assert!(result.is_err());
        }
    }

    mod write {
        use super::*;
        use tokio::fs;

        #[tokio::test]
        async fn test_write_image() {
            let adapter = LocalFileAdapter;
            let path = image_path("tmp_test_output.png");

            let image = dummy_image();
            let result = adapter.write(&path, &image).await;
            assert!(
                result.is_ok(),
                "Expected successful write, got: {:?}",
                result.err()
            );

            assert!(Path::new(&path).exists(), "Output file not created");
            fs::remove_file(&path).await.expect("Cleanup failed");
        }

        #[tokio::test]
        async fn test_write_failure_on_invalid_path() {
            let adapter = LocalFileAdapter;
            let image = dummy_image();
            let path = "/invalid_path/image.png";

            let result = adapter.write(path, &image).await;
            assert!(
                result.is_err(),
                "Expected error when writing to invalid path"
            );
        }
    }
}
