use std::sync::Arc;

use image::{DynamicImage, ImageFormat};

use crate::ImagePipeline;
use crate::services::encoding::image_encoder::ImageEncoder;
use crate::utils::format::infer_format_from_path;

impl ImagePipeline<'_> {
    /*
     * Get format from options
     * If not set, try to infer from the output file extension
     * If not set, try to infer from the input file extension to keep the same format
     */
    fn target_format(&self) -> ImageFormat {
        self.options
            .format
            .as_ref()
            .and_then(|f| f.get_format())
            .or_else(|| infer_format_from_path(&self.output))
            .or_else(|| infer_format_from_path(self.input))
            .unwrap_or(ImageFormat::Jpeg)
    }

    pub fn resolve_encoder(&self) -> Result<&Arc<dyn ImageEncoder>, anyhow::Error> {
        let target_format = self.target_format();

        if let Some(path_format) = infer_format_from_path(&self.output) {
            if self.options.format.is_some() && path_format != target_format {
                return Err(anyhow::anyhow!(
                    "[core/pipeline] Output format does not match the specified format"
                ));
            }
        }

        self.resolve_encoder_by_format(target_format)
    }

    pub fn resolve_encoder_by_format(
        &self,
        format: ImageFormat,
    ) -> Result<&Arc<dyn ImageEncoder>, anyhow::Error> {
        self.codec_resolver.resolve(format).ok_or_else(|| {
            anyhow::anyhow!(
                "[core/pipeline] No encoder registered for format: {}",
                format.to_mime_type()
            )
        })
    }

    pub fn get_image(&self) -> Result<&DynamicImage, anyhow::Error> {
        self.image
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("[core/pipeline] Image not loaded"))
    }
}
