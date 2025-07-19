use crate::ImagePipeline;
use image::{DynamicImage, GenericImageView};
use image::{GenericImage, Pixel, Rgba};

impl ImagePipeline<'_> {
    pub fn image_adjustments(&mut self) -> Result<(), anyhow::Error> {
        let opts = self.options.image_adjustments.clone().unwrap();
        let image = self
            .image
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("[core/image_adjustments] Image cannot be loaded"))?;

        if opts.grayscale {
            *image = image.grayscale();
        }

        if let Some(b) = opts.brightness {
            *image = image.brighten(b);
        }

        if let Some(c) = opts.contrast {
            *image = image.adjust_contrast(c);
        }

        if let Some(g) = opts.gamma {
            apply_gamma(image, g);
        }

        if opts.invert {
            image.invert();
        }

        if let Some(s) = opts.sharpen {
            *image = image.unsharpen(s.sigma, s.threshold);
        }

        Ok(())
    }
}

fn apply_gamma(image: &mut DynamicImage, gamma: f32) {
    let mut lut = [0u8; 256];
    let inv_gamma = 1.0 / gamma;
    #[allow(clippy::needless_range_loop)]
    for i in 0..256 {
        let normalized = (i as f32 / 255.0).powf(inv_gamma);
        lut[i] = (normalized * 255.0).clamp(0.0, 255.0) as u8;
    }

    let w = image.width();
    let h = image.height();

    let mut out = DynamicImage::new_rgba8(w, h);
    for (x, y, pixel) in image.pixels() {
        let channels = pixel.to_rgba().0;
        let corrected = Rgba([
            lut[channels[0] as usize],
            lut[channels[1] as usize],
            lut[channels[2] as usize],
            channels[3],
        ]);
        out.put_pixel(x, y, corrected);
    }

    *image = out
}
