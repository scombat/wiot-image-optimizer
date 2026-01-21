use std::sync::Arc;

use anyhow::{Result, bail};
use wiot_core::services::io::{FileAdapterFactory, FileDestination, FileSource};

use crate::http::HttpFileAdapterFactory;
use crate::local::LocalFileAdapter;

pub struct AdapterResolver;

impl AdapterResolver {
    // List of available adapters
    const ADAPTERS: [&'static dyn FileAdapterFactory; 2] =
        [&LocalFileAdapter, &HttpFileAdapterFactory];

    pub fn resolve_source(source: &str) -> Result<Arc<dyn FileSource>> {
        for adapter in Self::ADAPTERS {
            if adapter.can_handle(source) {
                return adapter.create_source(source);
            }
        }
        // TODO: Fallback to local file adapter if no other adapter is found
        bail!("No suitable adapter found for source: {}", source)
    }

    pub fn resolve_destination(destination: &str) -> Result<Arc<dyn FileDestination>> {
        for adapter in Self::ADAPTERS {
            if adapter.can_handle(destination) {
                return adapter.create_destination(destination);
            }
        }
        // TODO: Fallback to local file adapter if no other adapter is found
        bail!("No suitable adapter found for destination: {}", destination)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use wiot_core::services::io::{FileDestination, FileSource};

    fn is_local_file_adapter(source: &Arc<dyn FileSource>) -> bool {
        source.as_any().is::<LocalFileAdapter>()
    }

    fn is_local_file_adapter_destination(destination: &Arc<dyn FileDestination>) -> bool {
        destination.as_any().is::<LocalFileAdapter>()
    }

    #[test]
    fn test_local_resolver() {
        let io = vec![
            "file:///path/to/image.jpg",
            "file://path/to/image.jpg",
            "/path/to/image.jpg",
            "image.jpg",
            "./image.jpg",
            ".image.jpg",
            "../..image.jpg",
        ];

        for path in io {
            let source = AdapterResolver::resolve_source(path);
            assert!(
                source.is_ok(),
                "Failed to resolve source for path: {}",
                path
            );
            assert!(
                is_local_file_adapter(&source.unwrap()),
                "Source is not of type LocalFileAdapter"
            );

            let destination = AdapterResolver::resolve_destination(path);
            assert!(
                destination.is_ok(),
                "Failed to resolve destination for path: {}",
                path
            );
            assert!(
                is_local_file_adapter_destination(&destination.unwrap()),
                "Destination is not of type LocalFileAdapter"
            );
        }
    }

    #[test]
    #[ignore]
    fn test_not_local_resolver() {
        // TODO: implement when other adapters are available
        let invalid_paths = vec![
            "http://path/to/image.jpg",
            "https://path/to/image.jpg",
            "ftp://path/to/image.jpg",
            "sftp://path/to/image.jpg",
            "ws://path/to/image.jpg",
            "wss://path/to/image.jpg",
            "s3://path/to/image.jpg",
        ];

        for path in invalid_paths {
            let source = AdapterResolver::resolve_source(path);
            assert!(
                !is_local_file_adapter(&source.unwrap()),
                "Source should not be of type LocalFileAdapter"
            );

            let destination = AdapterResolver::resolve_destination(path);
            assert!(
                !is_local_file_adapter_destination(&destination.unwrap()),
                "Destination should not be of type LocalFileAdapter"
            );
        }
        unimplemented!();
    }

    #[test]
    fn test_resolve_source_fails_with_unknown_scheme() {
        let result = AdapterResolver::resolve_source("unknown://path/to/file.jpg");
        assert!(result.is_err(), "Expected error but got Ok");

        let message = result.err().unwrap().to_string();
        assert!(
            message.contains("No suitable adapter found for source"),
            "Unexpected error message: {message}"
        );
    }

    #[test]
    fn test_resolve_destination_fails_with_unknown_scheme() {
        let result = AdapterResolver::resolve_destination("strange://file.png");
        assert!(result.is_err(), "Expected error but got Ok");

        let message = result.err().unwrap().to_string();
        assert!(
            message.contains("No suitable adapter found for destination"),
            "Unexpected error message: {message}"
        );
    }
}
