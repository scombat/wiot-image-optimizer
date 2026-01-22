use crate::models::options::GravityOptions;
use crate::{ImagePipeline, utils::geometry::Geometry};

impl ImagePipeline<'_> {
    #[allow(clippy::collapsible_if)]
    pub fn crop(&mut self) -> Result<(), anyhow::Error> {
        let gravity = self.options.gravity.unwrap_or(GravityOptions::TopLeft);
        if let Some(ref options) = self.options.crop {
            if options.is_enabled() {
                // Get image
                let image = self
                    .image
                    .as_mut()
                    .ok_or_else(|| anyhow::anyhow!("[core/crop] Image cannot be loaded"))?;

                let area = Geometry::new(image).area(options, &gravity);
                let new_img = image.crop_imm(area.x, area.y, area.width, area.height);
                *image = new_img;
            }
        }
        Ok(())
    }
}
