use crate::ImagePipeline;

impl ImagePipeline<'_> {
    #[allow(clippy::collapsible_if)]
    pub fn blur(&mut self) -> Result<(), anyhow::Error> {
        if let Some(ref options) = self.options.blur {
            if options.is_enabled() {
                let image = self
                    .image
                    .as_mut()
                    .ok_or_else(|| anyhow::anyhow!("[core/Blur] Image cannot be loaded"))?;

                if let Some(sigma) = options.blur {
                    let new_img = image.blur(sigma);
                    *image = new_img
                }
            }
        }
        Ok(())
    }
}
