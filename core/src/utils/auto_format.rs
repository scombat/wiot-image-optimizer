use image::ImageFormat;

#[derive(Clone)]
pub struct FormatInfo {
    pub format: ImageFormat,
    pub supports_transparency: bool,
    pub is_lossless: bool,
    pub is_modern: bool,
}

pub static DEFAULT_AUTO_FORMATS: &[FormatInfo] = &[
    FormatInfo {
        format: ImageFormat::Avif,
        supports_transparency: true,
        is_lossless: true,
        is_modern: true,
    },
    FormatInfo {
        format: ImageFormat::WebP,
        supports_transparency: true,
        is_lossless: false,
        is_modern: true,
    },
    FormatInfo {
        format: ImageFormat::Jpeg,
        supports_transparency: false,
        is_lossless: false,
        is_modern: false,
    },
    FormatInfo {
        format: ImageFormat::Png,
        supports_transparency: true,
        is_lossless: true,
        is_modern: false,
    },
];
