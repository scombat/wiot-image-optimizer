use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{
    ImagePipeline,
    utils::auto_format::{DEFAULT_AUTO_FORMATS, FormatInfo},
};

impl ImagePipeline<'_> {
    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        self.load().await?;
        self.process()?;

        match self.options.auto_select_format {
            true => self.encode_auto_format().await?,
            _ => self.encode().await?,
        }

        self.store().await?;
        Ok(())
    }

    pub async fn load(&mut self) -> Result<(), anyhow::Error> {
        self.image = Some(self.source.read(self.input).await?);
        if self.image.is_none() {
            return Err(anyhow::anyhow!("[core/pipeline] Image cannot be loaded"));
        }
        Ok(())
    }

    pub async fn store(&mut self) -> Result<(), anyhow::Error> {
        let encoded = self.encoded_bytes.as_ref().ok_or_else(|| {
            anyhow::anyhow!("[core/pipeline] No encoded image available for storage")
        })?;
        self.destination.write(&self.output, encoded).await?;
        Ok(())
    }

    pub async fn encode(&mut self) -> Result<(), anyhow::Error> {
        let image = self
            .image
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No image loaded"))?;
        let encoder = self.resolve_encoder()?;
        let encoded = encoder.encode(image, &self.options)?;
        self.encoded_bytes = Some(encoded.clone());

        Ok(())
    }

    pub async fn encode_auto_format(&mut self) -> Result<(), anyhow::Error> {
        let mut handles = Vec::new();
        let image = self
            .image
            .clone()
            .ok_or_else(|| anyhow::anyhow!("No image loaded"))?;
        let has_alpha = image.color().has_alpha();
        let image_arc = Arc::new(image);

        // Filter formats against input image type
        let auto_formats: Vec<FormatInfo> = if has_alpha {
            DEFAULT_AUTO_FORMATS
                .iter()
                .filter(|f| f.supports_transparency)
                .cloned()
                .collect()
        } else {
            DEFAULT_AUTO_FORMATS.to_vec()
        };

        // For each format, get the encoder and spwan a thread to encode the original image
        for format_info in auto_formats {
            let encoder = self
                .resolve_encoder_by_format(format_info.format)
                .map_err(|e| {
                    anyhow::anyhow!("No encoder for format {:?}: {}", format_info.format, e)
                })?
                .clone();
            let image_clone = Arc::clone(&image_arc);
            let options = self.options.clone();

            // Use Arc<Mutex<Vec<u8>>> as buffer
            let buffer = Arc::new(Mutex::new(Vec::new()));
            let buffer_clone = Arc::clone(&buffer);

            let handle = thread::spawn(move || {
                let encoded = encoder.encode(&image_clone, &options)?;
                let mut buf = buffer_clone.lock().unwrap();
                buf.extend_from_slice(&encoded);
                Ok::<_, anyhow::Error>(())
            });

            handles.push((handle, buffer, format_info.format));
        }

        // Find the smallest encoded image
        let mut min_size: Option<usize> = None;
        let mut min_bytes: Option<Vec<u8>> = None;
        let mut min_format: Option<image::ImageFormat> = None;

        for (handle, buffer, format) in handles.into_iter() {
            handle
                .join()
                .map_err(|e| anyhow::anyhow!("Thread panicked: {:?}", e))??;
            let buf = buffer.lock().unwrap();
            let size = buf.len();
            if min_size.is_none() || size < min_size.unwrap() {
                min_size = Some(size);
                min_bytes = Some(buf.clone());
                min_format = Some(format);
            }
        }

        if let Some(bytes) = min_bytes {
            self.encoded_bytes = Some(bytes.clone());

            // Get encoded image bytes checksum
            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            let checksum = hasher.finalize();
            let checksum_hex = format!("{:x}", checksum);

            // Mute self.output to replace filename or concat file path by the right image ext.
            if let Some(format) = min_format {
                let ext = match format.extensions_str().first() {
                    Some(e) => *e,
                    None => "img",
                };
                let orig_path = Path::new(&self.output);

                // Determine if orig_path is a directory or a file path
                let target_dir =
                    if self.output.ends_with(std::path::MAIN_SEPARATOR) || orig_path.is_dir() {
                        // Explicitly a directory (trailing slash or detected as dir)
                        orig_path
                    } else {
                        // If it's a file path, use its parent directory (or current dir if none)
                        orig_path.parent().unwrap_or_else(|| Path::new(""))
                    };

                let new_filename = format!("{}.{ext}", checksum_hex);
                let new_path = target_dir.join(new_filename);
                self.output = new_path.to_string_lossy().to_string();
            }
        }
        Ok(())
    }
}
