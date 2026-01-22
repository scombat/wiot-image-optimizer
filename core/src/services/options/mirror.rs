use crate::ImagePipeline;

impl ImagePipeline<'_> {
    #[allow(clippy::collapsible_if)]
    pub fn mirror(&mut self) -> Result<(), anyhow::Error> {
        if let Some(ref options) = self.options.mirror {
            if options.is_enabled() {
                let image = self
                    .image
                    .as_mut()
                    .ok_or_else(|| anyhow::anyhow!("[core/mirror] Image cannot be loaded"))?;

                if options.flip {
                    *image = image.fliph();
                }
                if options.flop {
                    *image = image.flipv();
                }
            }
        }
        Ok(())
    }
}
