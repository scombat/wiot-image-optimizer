use clap::Args;

use wiot_core::models::options::BackgroundOptions;

#[derive(Args, Debug)]
pub struct BackgroundArgs {
    /// Background color for transparent images when converting to formats without transparency
    /// support (e.g., JPEG). Accepts hex (#FFFFFF or FFFFFF) or RGB (255,255,255) format.
    #[arg(long, default_value = "#FFFFFF")]
    pub background_color: String,
}

impl BackgroundArgs {
    pub fn get(&self) -> Option<BackgroundOptions> {
        match parse_color(&self.background_color) {
            Some((r, g, b)) => Some(BackgroundOptions::from_rgb(r, g, b)),
            None => {
                eprintln!(
                    "Warning: Invalid background color '{}', using default white",
                    self.background_color
                );
                None
            }
        }
    }
}

/// Parses a color string in hex (#FFFFFF or FFFFFF) or RGB (255,255,255) format.
/// Returns Some((r, g, b)) on success, None on failure.
fn parse_color(color: &str) -> Option<(u8, u8, u8)> {
    let trimmed = color.trim();

    // Try hex format (#FFFFFF or FFFFFF)
    if let Some(hex) = trimmed.strip_prefix('#') {
        return parse_hex(hex);
    }
    if trimmed.len() == 6 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return parse_hex(trimmed);
    }

    // Try RGB format (255,255,255)
    if trimmed.contains(',') {
        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.len() == 3 {
            let r = parts[0].trim().parse::<u8>().ok()?;
            let g = parts[1].trim().parse::<u8>().ok()?;
            let b = parts[2].trim().parse::<u8>().ok()?;
            return Some((r, g, b));
        }
    }

    None
}

fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_color {
        use super::*;

        #[test]
        fn parses_hex_with_hash() {
            assert_eq!(parse_color("#FFFFFF"), Some((255, 255, 255)));
            assert_eq!(parse_color("#000000"), Some((0, 0, 0)));
            assert_eq!(parse_color("#FF5733"), Some((255, 87, 51)));
        }

        #[test]
        fn parses_hex_without_hash() {
            assert_eq!(parse_color("FFFFFF"), Some((255, 255, 255)));
            assert_eq!(parse_color("000000"), Some((0, 0, 0)));
            assert_eq!(parse_color("ff5733"), Some((255, 87, 51)));
        }

        #[test]
        fn parses_rgb_format() {
            assert_eq!(parse_color("255,255,255"), Some((255, 255, 255)));
            assert_eq!(parse_color("0,0,0"), Some((0, 0, 0)));
            assert_eq!(parse_color("255, 87, 51"), Some((255, 87, 51)));
        }

        #[test]
        fn returns_none_for_invalid_input() {
            assert_eq!(parse_color("invalid"), None);
            assert_eq!(parse_color("#GGG"), None);
            assert_eq!(parse_color("256,0,0"), None);
            assert_eq!(parse_color(""), None);
            assert_eq!(parse_color("red"), None);
        }
    }

    mod background_args {
        use super::*;

        #[test]
        fn get_returns_options_for_valid_color() {
            let args = BackgroundArgs {
                background_color: "#FF0000".to_string(),
            };
            let opts = args.get();
            assert!(opts.is_some());
        }

        #[test]
        fn get_returns_none_for_invalid_color() {
            let args = BackgroundArgs {
                background_color: "invalid".to_string(),
            };
            let opts = args.get();
            assert!(opts.is_none());
        }
    }
}
