use anyhow::{Result, anyhow};
use clap::Args;
use image::ImageFormat;
use wiot_core::models::options::FormatOptions;

#[derive(Args, Debug)]
pub struct FormatArgs {
    /// Output image format (jpeg, png, webp, avif, gif)
    ///
    /// If not specified, the format will be inferred from the output file extension,
    /// unless --auto-format is enabled.
    #[arg(long, value_name = "FORMAT", conflicts_with = "auto_format")]
    pub format: Option<String>,

    /// Enable automatic selection of the output format.
    ///
    /// The format will be chosen to produce the smallest image.
    /// Default formats are: WebP, Avif, and Jpeg or Png depending on whether the image contains transparency.
    /// This flag is incompatible with the "format" option.
    #[arg(long, conflicts_with_all = &["format"])]
    pub auto_format: bool,
}

impl FormatArgs {
    pub fn get(&self) -> Result<Option<FormatOptions>> {
        match &self.format {
            None => Ok(None),
            Some(fmt) => {
                let image_format = ImageFormat::from_extension(fmt).ok_or_else(|| {
                    anyhow!(
                        "[cli/format] Unsupported format '{}'. Supported formats: \
                             jpeg, jpg, png, webp, avif, gif",
                        fmt
                    )
                })?;
                let opts = FormatOptions::new(Some(image_format))?;
                Ok(Some(opts))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_args_get_none() {
        let args = FormatArgs {
            format: None,
            auto_format: false,
        };
        let result = args.get().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_format_args_get_webp() {
        let args = FormatArgs {
            format: Some("webp".to_string()),
            auto_format: false,
        };
        let opts = args.get().unwrap().unwrap();
        assert_eq!(opts.format, Some(ImageFormat::WebP));
    }

    #[test]
    fn test_format_args_get_jpeg() {
        let args = FormatArgs {
            format: Some("jpeg".to_string()),
            auto_format: false,
        };
        let opts = args.get().unwrap().unwrap();
        assert_eq!(opts.format, Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_format_args_get_jpg() {
        let args = FormatArgs {
            format: Some("jpg".to_string()),
            auto_format: false,
        };
        let opts = args.get().unwrap().unwrap();
        assert_eq!(opts.format, Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_format_args_get_png() {
        let args = FormatArgs {
            format: Some("png".to_string()),
            auto_format: false,
        };
        let opts = args.get().unwrap().unwrap();
        assert_eq!(opts.format, Some(ImageFormat::Png));
    }

    #[test]
    fn test_format_args_get_avif() {
        let args = FormatArgs {
            format: Some("avif".to_string()),
            auto_format: false,
        };
        let opts = args.get().unwrap().unwrap();
        assert_eq!(opts.format, Some(ImageFormat::Avif));
    }

    #[test]
    fn test_format_args_get_gif() {
        let args = FormatArgs {
            format: Some("gif".to_string()),
            auto_format: false,
        };
        let opts = args.get().unwrap().unwrap();
        assert_eq!(opts.format, Some(ImageFormat::Gif));
    }

    #[test]
    fn test_format_args_get_invalid_unknown() {
        let args = FormatArgs {
            format: Some("xyz".to_string()),
            auto_format: false,
        };
        let result = args.get();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("[cli/format] Unsupported format 'xyz'")
        );
    }

    #[test]
    fn test_format_args_get_invalid_unsupported() {
        let args = FormatArgs {
            format: Some("bmp".to_string()),
            auto_format: false,
        };
        let result = args.get();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unsupported output format")
        );
    }

    #[test]
    fn test_format_args_get_case_insensitive() {
        let cases = vec![
            ("WEBP", ImageFormat::WebP),
            ("WebP", ImageFormat::WebP),
            ("PNG", ImageFormat::Png),
            ("Png", ImageFormat::Png),
            ("JPEG", ImageFormat::Jpeg),
            ("JPG", ImageFormat::Jpeg),
            ("AVIF", ImageFormat::Avif),
            ("GIF", ImageFormat::Gif),
        ];
        for (input, expected) in cases {
            let args = FormatArgs {
                format: Some(input.to_string()),
                auto_format: false,
            };
            let result = args.get();
            assert!(
                result.is_ok(),
                "Expected '{}' to be accepted, got: {}",
                input,
                result.unwrap_err()
            );
            assert_eq!(
                result.unwrap().unwrap().format,
                Some(expected),
                "Failed for input: {}",
                input
            );
        }
    }
}
