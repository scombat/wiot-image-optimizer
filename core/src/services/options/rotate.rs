use crate::ImagePipeline;
use image::DynamicImage;
use imageproc::geometric_transformations::{Interpolation, rotate_about_center};

impl ImagePipeline<'_> {
    #[allow(clippy::collapsible_if)]
    pub fn rotate(&mut self) -> Result<(), anyhow::Error> {
        if let Some(ref options) = self.options.rotate {
            if options.is_enabled() {
                let image = self
                    .image
                    .as_mut()
                    .ok_or_else(|| anyhow::anyhow!("[core/rotate] Image cannot be loaded"))?;

                let new_img = image.to_rgba8();

                let theta = options.angle_degrees.unwrap_or(0.0).to_radians();

                let rotated = rotate_about_center(
                    &new_img,
                    theta,
                    Interpolation::Bilinear,
                    image::Rgba([0, 0, 0, 0]),
                );

                *image = DynamicImage::ImageRgba8(rotated);
            }
        }
        Ok(())
    }
}
