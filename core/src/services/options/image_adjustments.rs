use image::DynamicImage;
use rayon::prelude::*;

use crate::ImagePipeline;

impl ImagePipeline<'_> {
    pub fn image_adjustments(&mut self) -> Result<(), anyhow::Error> {
        if let Some(opts) = &self.options.image_adjustments {
            if opts.is_enabled() {
                let image = self.image.as_mut().ok_or_else(|| {
                    anyhow::anyhow!("[core/image_adjustments] Image cannot be loaded")
                })?;

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

                if let Some(s) = &opts.sharpen {
                    *image = image.unsharpen(s.sigma, s.threshold);
                }
            }
        }

        Ok(())
    }
}

fn apply_gamma(image: &mut DynamicImage, gamma: f32) {
    let mut lut = [0u8; 256];
    let inv_gamma = 1.0 / gamma;
    #[allow(clippy::needless_range_loop)]
    lut.iter_mut().enumerate().for_each(|(i, lut_value)| {
        let normalized = (i as f32 / 255.0).powf(inv_gamma);
        *lut_value = (normalized * 255.0).clamp(0.0, 255.0) as u8;
    });

    let w = image.width();
    let h = image.height();

    let mut out = DynamicImage::new_rgba8(w, h);
    let mut out_buf = out.to_rgba8();
    let in_buf = image.to_rgba8();

    out_buf
        .par_chunks_mut(4)
        .zip(in_buf.par_chunks(4))
        .for_each(|(out_px, in_px)| {
            out_px[0] = lut[in_px[0] as usize];
            out_px[1] = lut[in_px[1] as usize];
            out_px[2] = lut[in_px[2] as usize];
            out_px[3] = in_px[3];
        });

    out = DynamicImage::ImageRgba8(out_buf);

    *image = out
}
