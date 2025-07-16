use anyhow::Error;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Coord {
    Px(i32),
    Percent(f32),
}

impl Coord {
    pub fn parse(s: &str) -> Result<Self, Error> {
        let s = s.trim();
        if s.contains('e') || s.contains('E') {
            return Err(anyhow::anyhow!("Scientific notation not allowed"));
        }
        if let Some(stripped) = s.strip_suffix('%') {
            let pct: f32 = stripped.parse()?;
            if !pct.is_finite() {
                return Err(anyhow::anyhow!("Should be a finite number"));
            }
            Ok(Coord::Percent(pct))
        } else if let Some(stripped) = s.strip_suffix("px") {
            let px: i32 = stripped.parse()?;
            Ok(Coord::Px(px))
        } else {
            let px: i32 = s.parse()?;
            Ok(Coord::Px(px))
        }
    }

    pub fn parse_opt(s: Option<&str>) -> Result<Option<Self>, Error> {
        match s {
            Some(val) => Ok(Some(Self::parse(val)?)),
            _ => Ok(None),
        }
    }

    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, Error> {
        Self::from_str(s.as_ref())
    }

    pub fn as_f32(&self) -> f32 {
        match self {
            Coord::Percent(p) => *p,
            Coord::Px(px) => *px as f32,
        }
    }
}

impl FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    fn valid_cases() -> Vec<(&'static str, Coord)> {
        vec![
            // Pixels, integer
            ("0", Coord::Px(0)),
            ("42", Coord::Px(42)),
            ("-17", Coord::Px(-17)),
            ("1234", Coord::Px(1234)),
            ("-999", Coord::Px(-999)),
            // Pixels, with px suffix
            ("0px", Coord::Px(0)),
            ("42px", Coord::Px(42)),
            ("-17px", Coord::Px(-17)),
            ("1234px", Coord::Px(1234)),
            ("-999px", Coord::Px(-999)),
            // Percent, integer
            ("0%", Coord::Percent(0.0)),
            ("100%", Coord::Percent(100.0)),
            ("50%", Coord::Percent(50.0)),
            ("-25%", Coord::Percent(-25.0)),
            ("200%", Coord::Percent(200.0)),
            // Percent, float
            ("12.5%", Coord::Percent(12.5)),
            ("-12.5%", Coord::Percent(-12.5)),
            ("99.99%", Coord::Percent(99.99)),
            ("-99.99%", Coord::Percent(-99.99)),
            // Spaces
            (" 42 ", Coord::Px(42)),
            (" 42px ", Coord::Px(42)),
            (" 42% ", Coord::Percent(42.0)),
            ("\t-17px\n", Coord::Px(-17)),
            (" 12.5% ", Coord::Percent(12.5)),
        ]
    }

    fn invalid_cases() -> Vec<&'static str> {
        vec![
            "",          // empty
            " ",         // whitespace only
            "abc",       // not a number
            "42percent", // invalid suffix
            "42 px",     // space in suffix
            "42%%",      // double percent
            "42px%",     // both px and percent
            "42.5.5px",  // invalid float
            "--42px",    // double negative
            "12,5%",     // comma instead of dot
            "px",        // only suffix
            "%",         // only percent
            "100p",      // unknown suffix
            "42pxpx",    // double px
            "42.0.0%",   // invalid float
            "1e2px",     // scientific notation not supported for px
            "1e2%",      // scientific notation not supported for percent
        ]
    }

    #[test]
    fn test_valid_coord_parsing() {
        for (input, expected) in valid_cases() {
            let parsed = Coord::parse(input).unwrap();
            assert_eq!(parsed, expected, "Failed to parse '{}'", input);

            // Also test FromStr
            let parsed2: Coord = input.parse().unwrap();
            assert_eq!(parsed2, expected, "FromStr failed for '{}'", input);
        }
    }

    #[test]
    fn test_invalid_coord_parsing() {
        for input in invalid_cases() {
            let parsed = Coord::parse(input);
            assert!(parsed.is_err(), "Should not parse '{}'", input);

            let parsed2: Result<Coord, _> = input.parse();
            assert!(parsed2.is_err(), "FromStr should not parse '{}'", input);
        }
    }

    #[test]
    fn fuzz_valid_percentages() {
        // Fuzz a range of percentages, including edge and negative values
        let values = [
            0.0, 1.0, -1.0, 0.5, -0.5, 2.0, -2.0, 99.999, -99.999, 100.0, -100.0, 12.34, -12.34,
        ];
        for v in values {
            let s = format!("{}%", v);
            let expected = Coord::Percent(v);
            let parsed = Coord::parse(&s).unwrap();
            assert!(
                (match parsed {
                    Coord::Percent(p) => (p - expected.as_f32()).abs() < 1e-6,
                    _ => false,
                }),
                "Failed to parse '{}', got {:?}",
                s,
                parsed
            );
        }
    }

    #[test]
    fn fuzz_valid_pixels() {
        // Fuzz a range of pixel values, including edge and negative values
        let values = [0, 1, -1, 100, -100, 99999, -99999, i32::MAX, i32::MIN + 1];
        for v in values {
            let s = v.to_string();
            let expected = Coord::Px(v);
            let parsed = Coord::parse(&s).unwrap();
            assert_eq!(parsed, expected, "Failed to parse '{}'", s);

            let s_px = format!("{}px", v);
            let parsed_px = Coord::parse(&s_px).unwrap();
            assert_eq!(parsed_px, expected, "Failed to parse '{}'", s_px);
        }
    }

    #[test]
    fn fuzz_invalid_inputs() {
        // Fuzz some random invalid strings
        let cases = [
            "foo%",
            "barpx",
            "42p",
            "42x",
            "42px%",
            "42%px",
            "42..5px",
            "42..5%",
            "++42px",
            "--42%",
            "42-42px",
            "1_000px",
            "1_000%",
            "NaN%",
            "infpx",
            "-inf%",
            "42.42.42px",
            "42.42.42%",
        ];
        for input in cases {
            assert!(Coord::parse(input).is_err(), "Should not parse '{}'", input);
        }
    }

    #[test]
    fn test_parse_opt() {
        assert_eq!(Coord::parse_opt(Some("42")).unwrap(), Some(Coord::Px(42)));
        assert_eq!(
            Coord::parse_opt(Some("50%")).unwrap(),
            Some(Coord::Percent(50.0))
        );
        assert_eq!(Coord::parse_opt(None).unwrap(), None);
        assert!(Coord::parse_opt(Some("notanumber")).is_err());
    }

    #[test]
    fn test_new() {
        assert_eq!(Coord::new("123").unwrap(), Coord::Px(123));
        assert_eq!(Coord::new("-99px").unwrap(), Coord::Px(-99));
        assert_eq!(Coord::new("25%").unwrap(), Coord::Percent(25.0));
        assert!(Coord::new("badinput").is_err());
    }

    #[test]
    fn test_as_f32() {
        let px = Coord::Px(42);
        let percent = Coord::Percent(0.75);
        assert_eq!(px.as_f32(), 42.0);
        assert!((percent.as_f32() - 0.75).abs() < 1e-6);
    }

    #[test]
    fn test_from_str() {
        use std::str::FromStr;
        assert_eq!(Coord::from_str("10px").unwrap(), Coord::Px(10));
        assert_eq!(Coord::from_str("10%").unwrap(), Coord::Percent(10.0));
        assert_eq!(Coord::from_str("-5").unwrap(), Coord::Px(-5));
        assert!(Coord::from_str("invalid").is_err());
    }
}
