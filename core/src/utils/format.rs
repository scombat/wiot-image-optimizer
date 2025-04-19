use image::ImageFormat;
use std::path::Path;

// Try to infer an `ImageFormat` from a given path or URI (file, http, etc.)
pub fn infer_format_from_path(path: &str) -> Option<ImageFormat> {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .and_then(ImageFormat::from_extension)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageFormat;

    #[test]
    fn test_infer_format_from_path() {
        let cases = vec![
            ("image.png", Some(ImageFormat::Png)),
            ("image.jpeg", Some(ImageFormat::Jpeg)),
            ("image.jpg", Some(ImageFormat::Jpeg)),
            ("image.webp", Some(ImageFormat::WebP)),
            ("image.avif", Some(ImageFormat::Avif)),
            ("image.bmp", Some(ImageFormat::Bmp)),
            ("image.tiff", Some(ImageFormat::Tiff)),
            ("image.txt", None),
            ("image", None),
            ("", None),
        ];

        for (path, expected) in cases {
            assert_eq!(infer_format_from_path(path), expected, "Failed on: {path}");
        }
    }
}
