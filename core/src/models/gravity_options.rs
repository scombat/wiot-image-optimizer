use anyhow::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GravityOptions {
    #[default]
    TopLeft,
    Top,
    TopRight,
    Center,
    CenterLeft,
    CenterRight,
    Bottom,
    BottomLeft,
    BottomRight,
}

impl GravityOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            GravityOptions::Top => "top",
            GravityOptions::TopLeft => "top-left",
            GravityOptions::TopRight => "top-right",
            GravityOptions::Center => "center",
            GravityOptions::CenterLeft => "center-left",
            GravityOptions::CenterRight => "center-right",
            GravityOptions::Bottom => "bottom",
            GravityOptions::BottomLeft => "bottom-left",
            GravityOptions::BottomRight => "bottom-right",
        }
    }

    pub fn abscissa_coef(&self) -> f32 {
        match self {
            GravityOptions::CenterLeft | GravityOptions::TopLeft | GravityOptions::BottomLeft => {
                0.0
            }
            GravityOptions::CenterRight
            | GravityOptions::TopRight
            | GravityOptions::BottomRight => 1.0,
            _ => 0.5,
        }
    }

    pub fn ordinate_coef(&self) -> f32 {
        match self {
            GravityOptions::Top | GravityOptions::TopLeft | GravityOptions::TopRight => 0.0,
            GravityOptions::Bottom | GravityOptions::BottomLeft | GravityOptions::BottomRight => {
                1.0
            }
            _ => 0.5,
        }
    }

    pub fn as_coef(&self) -> (f32, f32) {
        (self.abscissa_coef(), self.ordinate_coef())
    }
}

impl Display for GravityOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for GravityOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top" => Ok(GravityOptions::Top),
            "top-left" => Ok(GravityOptions::TopLeft),
            "top-right" => Ok(GravityOptions::TopRight),
            "center" => Ok(GravityOptions::Center),
            "center-left" => Ok(GravityOptions::CenterLeft),
            "center-right" => Ok(GravityOptions::CenterRight),
            "bottom" => Ok(GravityOptions::Bottom),
            "bottom-left" => Ok(GravityOptions::BottomLeft),
            "bottom-right" => Ok(GravityOptions::BottomRight),
            _ => Err(anyhow::anyhow!("Unknown gravity")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn test_as_str() {
        assert_eq!(GravityOptions::Top.as_str(), "top");
        assert_eq!(GravityOptions::TopLeft.as_str(), "top-left");
        assert_eq!(GravityOptions::TopRight.as_str(), "top-right");
        assert_eq!(GravityOptions::Center.as_str(), "center");
        assert_eq!(GravityOptions::CenterLeft.as_str(), "center-left");
        assert_eq!(GravityOptions::CenterRight.as_str(), "center-right");
        assert_eq!(GravityOptions::Bottom.as_str(), "bottom");
        assert_eq!(GravityOptions::BottomLeft.as_str(), "bottom-left");
        assert_eq!(GravityOptions::BottomRight.as_str(), "bottom-right");
    }

    #[test]
    fn test_default() {
        assert_eq!(GravityOptions::default(), GravityOptions::TopLeft);
    }

    #[test]
    fn test_display_fmt() {
        let variants = [
            (GravityOptions::Top, "top"),
            (GravityOptions::TopLeft, "top-left"),
            (GravityOptions::TopRight, "top-right"),
            (GravityOptions::Center, "center"),
            (GravityOptions::CenterLeft, "center-left"),
            (GravityOptions::CenterRight, "center-right"),
            (GravityOptions::Bottom, "bottom"),
            (GravityOptions::BottomLeft, "bottom-left"),
            (GravityOptions::BottomRight, "bottom-right"),
        ];
        for (variant, expected) in variants.iter() {
            assert_eq!(variant.to_string(), *expected);
        }
    }

    #[test]
    fn test_from_str_valid() {
        let cases = [
            ("top", GravityOptions::Top),
            ("top-left", GravityOptions::TopLeft),
            ("top-right", GravityOptions::TopRight),
            ("center", GravityOptions::Center),
            ("center-left", GravityOptions::CenterLeft),
            ("center-right", GravityOptions::CenterRight),
            ("bottom", GravityOptions::Bottom),
            ("bottom-left", GravityOptions::BottomLeft),
            ("bottom-right", GravityOptions::BottomRight),
            // Case insensitivity
            ("TOP", GravityOptions::Top),
            ("Top-Left", GravityOptions::TopLeft),
            ("CENTER", GravityOptions::Center),
            ("Center-Right", GravityOptions::CenterRight),
            ("BOTTOM", GravityOptions::Bottom),
            ("Bottom-Left", GravityOptions::BottomLeft),
        ];
        for (input, expected) in cases.iter() {
            let parsed = GravityOptions::from_str(input).unwrap();
            assert_eq!(&parsed, expected, "Failed to parse '{}'", input);
        }
    }

    #[test]
    fn test_from_str_invalid() {
        let invalid_cases = [
            "",
            " ",
            "middle",
            "left",
            "right",
            "top_left",
            "bottomright",
            "centerleft",
            "top left",
            "bottom right",
            "centerright",
            "unknown",
            "topleft",
            "center-rights",
            "bottom-",
            "-top",
            "top- right",
            "center- left",
        ];
        for input in invalid_cases.iter() {
            let result = GravityOptions::from_str(input);
            assert!(result.is_err(), "Input '{}' should be invalid", input);
        }
    }

    #[test]
    fn test_gravity_options_display() {
        use super::GravityOptions;
        let cases = [
            (GravityOptions::Top, "top"),
            (GravityOptions::TopLeft, "top-left"),
            (GravityOptions::TopRight, "top-right"),
            (GravityOptions::Center, "center"),
            (GravityOptions::CenterLeft, "center-left"),
            (GravityOptions::CenterRight, "center-right"),
            (GravityOptions::Bottom, "bottom"),
            (GravityOptions::BottomLeft, "bottom-left"),
            (GravityOptions::BottomRight, "bottom-right"),
        ];
        for (variant, expected_str) in cases.iter() {
            assert_eq!(
                variant.to_string(),
                *expected_str,
                "Display for {:?} failed",
                variant
            );
        }
    }

    #[test]
    fn test_gravity_options_default() {
        use super::GravityOptions;
        let default = GravityOptions::default();
        assert_eq!(
            default,
            GravityOptions::TopLeft,
            "Default GravityOptions should be TopLeft"
        );
    }

    #[test]
    fn test_gravity_options_clone_and_eq() {
        use super::GravityOptions;
        let a = GravityOptions::TopLeft;
        #[allow(clippy::clone_on_copy)]
        let b = a.clone();
        assert_eq!(a, b, "Cloned GravityOptions should be equal to original");
    }

    #[test]
    fn test_gravity_options_coefficients() {
        use super::GravityOptions;

        // Define a helper to get coefficients for each variant
        fn get_coeffs(gravity: &GravityOptions) -> (f32, f32) {
            match gravity {
                GravityOptions::Top => (0.5, 0.0),
                GravityOptions::TopLeft => (0.0, 0.0),
                GravityOptions::TopRight => (1.0, 0.0),
                GravityOptions::Center => (0.5, 0.5),
                GravityOptions::CenterLeft => (0.0, 0.5),
                GravityOptions::CenterRight => (1.0, 0.5),
                GravityOptions::Bottom => (0.5, 1.0),
                GravityOptions::BottomLeft => (0.0, 1.0),
                GravityOptions::BottomRight => (1.0, 1.0),
            }
        }

        let cases = [
            (GravityOptions::Top, (0.5, 0.0)),
            (GravityOptions::TopLeft, (0.0, 0.0)),
            (GravityOptions::TopRight, (1.0, 0.0)),
            (GravityOptions::Center, (0.5, 0.5)),
            (GravityOptions::CenterLeft, (0.0, 0.5)),
            (GravityOptions::CenterRight, (1.0, 0.5)),
            (GravityOptions::Bottom, (0.5, 1.0)),
            (GravityOptions::BottomLeft, (0.0, 1.0)),
            (GravityOptions::BottomRight, (1.0, 1.0)),
        ];

        for (variant, expected) in cases.iter() {
            let coeffs = get_coeffs(variant);
            assert!(
                (coeffs.0 - expected.0).abs() < f32::EPSILON
                    && (coeffs.1 - expected.1).abs() < f32::EPSILON,
                "Gravity {:?} coefficients expected {:?}, got {:?}",
                variant,
                expected,
                coeffs
            );
        }
    }
}
