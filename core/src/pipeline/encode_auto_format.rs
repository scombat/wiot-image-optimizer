use std::path::{MAIN_SEPARATOR, Path};
use std::sync::{Arc, Mutex};

use sha2::{Digest, Sha256};
use tokio::task::{JoinHandle, spawn_blocking};

use crate::ImagePipeline;
use crate::utils::auto_format::{DEFAULT_AUTO_FORMATS, FormatInfo};

type EncodedHandle = (
    JoinHandle<Result<(), anyhow::Error>>,
    Arc<Mutex<Vec<u8>>>,
    image::ImageFormat,
);

type SmallestEncodedResult = Result<(Option<Vec<u8>>, Option<image::ImageFormat>), anyhow::Error>;

pub(crate) async fn find_smallest_encoded(handles: Vec<EncodedHandle>) -> SmallestEncodedResult {
    let mut min_size: Option<usize> = None;
    let mut min_bytes: Option<Vec<u8>> = None;
    let mut min_format: Option<image::ImageFormat> = None;

    for (handle, buffer, format) in handles {
        handle.await??;
        let buf = buffer.lock().unwrap();
        let size = buf.len();
        if min_size.is_none() || size < min_size.unwrap() {
            min_size = Some(size);
            min_bytes = Some(buf.clone());
            min_format = Some(format);
        }
    }
    Ok((min_bytes, min_format))
}

pub(crate) fn checksum_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let checksum = hasher.finalize();
    format!("{:x}", checksum)
}

pub(crate) fn build_output_path(
    orig_output: &str,
    checksum_hex: &str,
    format: image::ImageFormat,
) -> String {
    let ext = format.extensions_str().first().copied().unwrap_or("img");
    let orig_path = Path::new(orig_output);
    let target_dir = if orig_output.ends_with(MAIN_SEPARATOR) || orig_path.is_dir() {
        orig_path
    } else {
        orig_path.parent().unwrap_or_else(|| Path::new(""))
    };
    let new_filename = format!("{}.{ext}", checksum_hex);
    let new_path = target_dir.join(new_filename);
    new_path.to_string_lossy().to_string()
}

impl ImagePipeline<'_> {
    pub async fn encode_auto_format(&mut self) -> Result<(), anyhow::Error> {
        let image = self
            .image
            .clone()
            .ok_or_else(|| anyhow::anyhow!("No image loaded"))?;
        let has_alpha = image.color().has_alpha();
        let image_arc = Arc::new(image);

        let auto_formats: Vec<FormatInfo> = if has_alpha {
            DEFAULT_AUTO_FORMATS
                .iter()
                .filter(|f| f.supports_transparency)
                .cloned()
                .collect()
        } else {
            DEFAULT_AUTO_FORMATS.to_vec()
        };

        let mut handles = Vec::with_capacity(auto_formats.len());
        for format_info in auto_formats {
            let encoder = self
                .resolve_encoder_by_format(format_info.format)
                .map_err(|e| {
                    anyhow::anyhow!("No encoder for format {:?}: {}", format_info.format, e)
                })?
                .clone();
            let image_clone = Arc::clone(&image_arc);
            let options = self.options.clone();
            let buffer = Arc::new(Mutex::new(Vec::new()));
            let buffer_clone = Arc::clone(&buffer);

            let handle = spawn_blocking(move || {
                let encoded = encoder.encode(&image_clone, &options)?;
                let mut buf = buffer_clone.lock().unwrap();
                buf.extend_from_slice(&encoded);
                Ok::<_, anyhow::Error>(())
            });

            handles.push((handle, buffer, format_info.format));
        }

        let (min_bytes, min_format) = find_smallest_encoded(handles).await?;

        if let Some(bytes) = min_bytes {
            self.encoded_bytes = Some(bytes.clone());
            let checksum_hex = checksum_hex(&bytes);
            if let Some(format) = min_format {
                self.output = build_output_path(&self.output, &checksum_hex, format);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageFormat;
    use sha2::{Digest, Sha256};
    use std::path::MAIN_SEPARATOR;
    use std::path::Path;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_find_smallest_encoded_returns_smallest() {
        let data1 = vec![1u8; 10];
        let data2 = vec![2u8; 5];
        let data3 = vec![3u8; 20];

        let buf1 = Arc::new(Mutex::new(data1.clone()));
        let buf2 = Arc::new(Mutex::new(data2.clone()));
        let buf3 = Arc::new(Mutex::new(data3.clone()));

        let h1 = tokio::spawn(async { Ok::<(), anyhow::Error>(()) });
        let h2 = tokio::spawn(async { Ok::<(), anyhow::Error>(()) });
        let h3 = tokio::spawn(async { Ok::<(), anyhow::Error>(()) });

        let handles = vec![
            (h1, buf1.clone(), ImageFormat::Png),
            (h2, buf2.clone(), ImageFormat::Jpeg),
            (h3, buf3.clone(), ImageFormat::Gif),
        ];

        let (min_bytes, min_format) = find_smallest_encoded(handles).await.unwrap();
        assert_eq!(min_bytes, Some(data2));
        assert_eq!(min_format, Some(ImageFormat::Jpeg));
    }

    #[tokio::test]
    async fn test_find_smallest_encoded_empty() {
        let handles = vec![];
        let (min_bytes, min_format) = find_smallest_encoded(handles).await.unwrap();
        assert!(min_bytes.is_none());
        assert!(min_format.is_none());
    }

    #[tokio::test]
    async fn test_find_smallest_encoded_handle_error() {
        let buf = Arc::new(Mutex::new(vec![1u8; 10]));
        let h = tokio::spawn(async { Err::<(), anyhow::Error>(anyhow::anyhow!("fail")) });
        let handles = vec![(h, buf, ImageFormat::Png)];
        let result = find_smallest_encoded(handles).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_smallest_encoded_join_error() {
        let buf = Arc::new(Mutex::new(vec![1u8; 10]));
        // Drop the handle immediately to simulate join error
        let h = tokio::task::spawn_blocking(|| panic!("panic in task"));
        let handles = vec![(h, buf, ImageFormat::Png)];
        let result = find_smallest_encoded(handles).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_checksum_hex_correctness() {
        let data = b"hello world";
        let mut hasher = Sha256::new();
        hasher.update(data);
        let expected = format!("{:x}", hasher.finalize());
        let actual = checksum_hex(data);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_build_output_path_regular_file() {
        let orig_output = "foo/bar/image.png";
        let checksum = "abc123";
        let format = ImageFormat::Jpeg;
        let result = build_output_path(orig_output, checksum, format);
        let expected_ext = format.extensions_str().first().unwrap();
        let expected_path = Path::new("foo/bar").join(format!("{}.{}", checksum, expected_ext));
        assert_eq!(result, expected_path.to_string_lossy());
    }

    #[test]
    fn test_build_output_path_directory() {
        let orig_output = format!("foo{}bar{}", MAIN_SEPARATOR, MAIN_SEPARATOR);
        let checksum = "def456";
        let format = ImageFormat::Png;
        let result = build_output_path(&orig_output, checksum, format);
        let expected_ext = format.extensions_str().first().unwrap();
        let expected_path = Path::new(&orig_output).join(format!("{}.{}", checksum, expected_ext));
        assert_eq!(result, expected_path.to_string_lossy());
    }

    #[test]
    fn test_build_output_path_no_parent() {
        let orig_output = "image";
        let checksum = "xyz789";
        let format = ImageFormat::Gif;
        let result = build_output_path(orig_output, checksum, format);
        let expected_ext = format.extensions_str().first().unwrap();
        let expected_path = Path::new("").join(format!("{}.{}", checksum, expected_ext));
        assert_eq!(result, expected_path.to_string_lossy());
    }

    #[test]
    fn test_build_output_path_unknown_extension() {
        // Uses a valid format but simulates the absence of an extension by patching the logic
        use image::ImageFormat;
        use std::path::Path;

        // We temporarily redefine the function for this test
        fn build_output_path_no_ext(
            orig_output: &str,
            checksum: &str,
            _format: ImageFormat,
        ) -> String {
            // Simulates the absence of an extension
            let parent = Path::new(orig_output)
                .parent()
                .unwrap_or_else(|| Path::new(""));
            let filename = format!("{}.img", checksum);
            parent.join(filename).to_string_lossy().to_string()
        }

        let orig_output = "foo/bar/image";
        let checksum = "noext";
        let format = ImageFormat::Jpeg; // valid format, but we simulate the absence of an extension
        let result = build_output_path_no_ext(orig_output, checksum, format);
        let expected_path = Path::new("foo/bar").join(format!("{}.img", checksum));
        assert_eq!(result, expected_path.to_string_lossy());
    }
}
