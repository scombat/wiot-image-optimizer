use std::io::Write;

use crate::models::options::ProcessingOptions;
use anyhow::Result;
use image::ImageEncoder as ImageEncoderTrait;
use image::{
    ColorType, DynamicImage, ImageFormat,
    codecs::png::{CompressionType, FilterType, PngEncoder as InnerPngEncoder},
};

use crate::services::encoding::image_encoder::ImageEncoder;

pub struct PngEncoder;

impl ImageEncoder for PngEncoder {
    fn encode(&self, image: &DynamicImage, _options: &ProcessingOptions) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.encode_to_writer(&mut buffer, image, _options)?;
        Ok(buffer)
    }

    fn encode_to_writer(
        &self,
        writer: &mut dyn Write,
        image: &DynamicImage,
        options: &ProcessingOptions,
    ) -> Result<()> {
        let (compression, filter) = PngEncoder::guess_encoding_params(options.get_quality());
        let (buffer, width, height, color_type) = PngEncoder::extract_buffer_and_type(image)?;
        let encoder = InnerPngEncoder::new_with_quality(writer, compression, filter);
        encoder.write_image(buffer, width, height, color_type.into())?;
        Ok(())
    }

    fn extension(&self) -> &'static str {
        self.format()
            .extensions_str()
            .first()
            .copied()
            .unwrap_or("png")
    }

    fn mime_type(&self) -> &'static str {
        self.format().to_mime_type()
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::Png
    }

    fn supports_native_quality_encoding(&self) -> bool {
        false
    }
}

impl PngEncoder {
    fn guess_encoding_params(quality: Option<f32>) -> (CompressionType, FilterType) {
        let compression = match quality.unwrap_or(75.0) as u8 {
            90..=100 => CompressionType::Fast,
            40..=89 => CompressionType::Default,
            _ => CompressionType::Best,
        };

        let filter = FilterType::Adaptive;
        (compression, filter)
    }

    pub fn extract_buffer_and_type(
        image: &DynamicImage,
    ) -> Result<(&[u8], u32, u32, ColorType), anyhow::Error> {
        match image {
            DynamicImage::ImageLuma8(buf) => {
                Ok((buf.as_raw(), buf.width(), buf.height(), ColorType::L8))
            }
            DynamicImage::ImageLumaA8(buf) => {
                Ok((buf.as_raw(), buf.width(), buf.height(), ColorType::La8))
            }
            DynamicImage::ImageRgb8(buf) => {
                Ok((buf.as_raw(), buf.width(), buf.height(), ColorType::Rgb8))
            }
            DynamicImage::ImageRgba8(buf) => {
                Ok((buf.as_raw(), buf.width(), buf.height(), ColorType::Rgba8))
            }
            DynamicImage::ImageLuma16(buf) => Ok((
                Self::as_u8_slice(buf),
                buf.width(),
                buf.height(),
                ColorType::L16,
            )),
            DynamicImage::ImageLumaA16(buf) => Ok((
                Self::as_u8_slice(buf),
                buf.width(),
                buf.height(),
                ColorType::La16,
            )),
            DynamicImage::ImageRgb16(buf) => Ok((
                Self::as_u8_slice(buf),
                buf.width(),
                buf.height(),
                ColorType::Rgb16,
            )),
            DynamicImage::ImageRgba16(buf) => Ok((
                Self::as_u8_slice(buf),
                buf.width(),
                buf.height(),
                ColorType::Rgba16,
            )),
            _ => Err(anyhow::anyhow!(
                "Unsupported DynamicImage variant for PNG encoding"
            )),
        }
    }

    /// For 16-bit buffers, convert &[u16] to &[u8]
    fn as_u8_slice<T: bytemuck::Pod>(buffer: &[T]) -> &[u8] {
        bytemuck::cast_slice(buffer)
    }
}
