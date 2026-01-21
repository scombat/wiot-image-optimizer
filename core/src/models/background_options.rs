use anyhow::{Result, anyhow};
use image::Rgba;

#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundOptions {
    pub color: Rgba<u8>,
}

impl Default for BackgroundOptions {
    fn default() -> Self {
        Self {
            color: Rgba([255, 255, 255, 255]),
        }
    }
}

impl BackgroundOptions {
    pub fn new(color: Rgba<u8>) -> Self {
        Self { color }
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');

        let (r, g, b) = match hex.len() {
            3 => {
                let chars: Vec<char> = hex.chars().collect();
                let r = Self::parse_hex_digit(chars[0])? * 17;
                let g = Self::parse_hex_digit(chars[1])? * 17;
                let b = Self::parse_hex_digit(chars[2])? * 17;
                (r, g, b)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| anyhow!("Invalid hex color: {}", hex))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| anyhow!("Invalid hex color: {}", hex))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| anyhow!("Invalid hex color: {}", hex))?;
                (r, g, b)
            }
            _ => return Err(anyhow!("Invalid hex color format: {}", hex)),
        };

        Ok(Self::from_rgb(r, g, b))
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            color: Rgba([r, g, b, 255]),
        }
    }

    pub fn parse(input: &str) -> Result<Self> {
        let input = input.trim();

        // Check if it looks like hex (starts with # or contains only hex chars)
        if input.starts_with('#') {
            return Self::from_hex(input);
        }

        // Check if it looks like RGB (contains comma)
        if input.contains(',') {
            let parts: Vec<&str> = input.split(',').map(|s| s.trim()).collect();
            if parts.len() != 3 {
                return Err(anyhow!(
                    "Invalid RGB format: expected 3 values, got {}",
                    parts.len()
                ));
            }

            let r = parts[0]
                .parse::<u8>()
                .map_err(|_| anyhow!("Invalid red value: {}", parts[0]))?;
            let g = parts[1]
                .parse::<u8>()
                .map_err(|_| anyhow!("Invalid green value: {}", parts[1]))?;
            let b = parts[2]
                .parse::<u8>()
                .map_err(|_| anyhow!("Invalid blue value: {}", parts[2]))?;

            return Ok(Self::from_rgb(r, g, b));
        }

        // Try hex without # prefix
        Self::from_hex(input)
    }

    fn parse_hex_digit(c: char) -> Result<u8> {
        c.to_digit(16)
            .map(|d| d as u8)
            .ok_or_else(|| anyhow!("Invalid hex digit: {}", c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod default {
        use super::*;

        #[test]
        fn returns_white() {
            let opts = BackgroundOptions::default();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }
    }

    mod new {
        use super::*;

        #[test]
        fn creates_with_given_color() {
            let color = Rgba([100, 150, 200, 255]);
            let opts = BackgroundOptions::new(color);
            assert_eq!(opts.color, color);
        }
    }

    mod from_hex {
        use super::*;

        #[test]
        fn parses_6_digit_with_hash() {
            let opts = BackgroundOptions::from_hex("#FFFFFF").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_6_digit_without_hash() {
            let opts = BackgroundOptions::from_hex("FFFFFF").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_3_digit_with_hash() {
            let opts = BackgroundOptions::from_hex("#fff").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_3_digit_without_hash() {
            let opts = BackgroundOptions::from_hex("fff").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_uppercase_3_digit() {
            let opts = BackgroundOptions::from_hex("#FFF").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_mixed_case() {
            let opts = BackgroundOptions::from_hex("#FfFfFf").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_black() {
            let opts = BackgroundOptions::from_hex("#000000").unwrap();
            assert_eq!(opts.color, Rgba([0, 0, 0, 255]));
        }

        #[test]
        fn parses_red() {
            let opts = BackgroundOptions::from_hex("#FF0000").unwrap();
            assert_eq!(opts.color, Rgba([255, 0, 0, 255]));
        }

        #[test]
        fn parses_3_digit_shorthand_correctly() {
            // #abc should expand to #aabbcc
            let opts = BackgroundOptions::from_hex("#abc").unwrap();
            assert_eq!(opts.color, Rgba([170, 187, 204, 255]));
        }

        #[test]
        fn invalid_hex_characters() {
            assert!(BackgroundOptions::from_hex("#GGGGGG").is_err());
        }

        #[test]
        fn invalid_length_too_short() {
            assert!(BackgroundOptions::from_hex("#FF").is_err());
        }

        #[test]
        fn invalid_length_too_long() {
            assert!(BackgroundOptions::from_hex("#FFFFFFF").is_err());
        }

        #[test]
        fn invalid_4_digit() {
            assert!(BackgroundOptions::from_hex("#FFFF").is_err());
        }

        #[test]
        fn invalid_5_digit() {
            assert!(BackgroundOptions::from_hex("#FFFFF").is_err());
        }
    }

    mod from_rgb {
        use super::*;

        #[test]
        fn creates_white() {
            let opts = BackgroundOptions::from_rgb(255, 255, 255);
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn creates_black() {
            let opts = BackgroundOptions::from_rgb(0, 0, 0);
            assert_eq!(opts.color, Rgba([0, 0, 0, 255]));
        }

        #[test]
        fn creates_custom_color() {
            let opts = BackgroundOptions::from_rgb(100, 150, 200);
            assert_eq!(opts.color, Rgba([100, 150, 200, 255]));
        }
    }

    mod parse {
        use super::*;

        #[test]
        fn parses_hex_with_hash() {
            let opts = BackgroundOptions::parse("#FFFFFF").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_hex_without_hash() {
            let opts = BackgroundOptions::parse("FFFFFF").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_rgb_no_spaces() {
            let opts = BackgroundOptions::parse("255,255,255").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_rgb_with_spaces() {
            let opts = BackgroundOptions::parse("255, 255, 255").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn parses_rgb_black() {
            let opts = BackgroundOptions::parse("0,0,0").unwrap();
            assert_eq!(opts.color, Rgba([0, 0, 0, 255]));
        }

        #[test]
        fn parses_rgb_with_extra_whitespace() {
            let opts = BackgroundOptions::parse("  100 , 150 , 200  ").unwrap();
            assert_eq!(opts.color, Rgba([100, 150, 200, 255]));
        }

        #[test]
        fn parses_3_digit_hex() {
            let opts = BackgroundOptions::parse("fff").unwrap();
            assert_eq!(opts.color, Rgba([255, 255, 255, 255]));
        }

        #[test]
        fn invalid_rgb_too_few_values() {
            assert!(BackgroundOptions::parse("255,255").is_err());
        }

        #[test]
        fn invalid_rgb_too_many_values() {
            assert!(BackgroundOptions::parse("255,255,255,255").is_err());
        }

        #[test]
        fn invalid_rgb_non_numeric() {
            assert!(BackgroundOptions::parse("255,abc,255").is_err());
        }

        #[test]
        fn invalid_rgb_out_of_range() {
            assert!(BackgroundOptions::parse("256,255,255").is_err());
        }

        #[test]
        fn invalid_rgb_negative() {
            assert!(BackgroundOptions::parse("-1,255,255").is_err());
        }
    }
}
