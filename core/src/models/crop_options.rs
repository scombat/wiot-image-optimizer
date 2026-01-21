use anyhow::Error;

use crate::utils::coordinates::Coord;

#[derive(Debug, Clone, PartialEq)]
pub struct CropOptions {
    pub width: Option<Coord>,
    pub height: Option<Coord>,
    pub offset_x: Option<Coord>,
    pub offset_y: Option<Coord>,
}

impl CropOptions {
    pub fn new(
        x: Option<&str>,
        y: Option<&str>,
        offset_x: Option<&str>,
        offset_y: Option<&str>,
    ) -> Result<Self, Error> {
        Ok(Self {
            width: Coord::parse_opt(x)?,
            height: Coord::parse_opt(y)?,
            offset_x: Coord::parse_opt(offset_x)?,
            offset_y: Coord::parse_opt(offset_y)?,
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.offset_x.is_some()
            || self.offset_y.is_some()
            || self.width.is_some()
            || self.height.is_some()
    }

    pub fn validate(&self) -> Result<(), Error> {
        // Validate x and y: if present, must be valid Coord (already parsed)
        // Validate offset_x and offset_y: if present, must be percent between -100% and +100% or any px
        let check_offset = |name: &str, coord: &Option<Coord>| -> Result<(), Error> {
            if let Some(Coord::Percent(p)) = coord {
                if *p < -100.0 || *p > 100.0 {
                    return Err(anyhow::anyhow!(
                        "{} percent must be between -100% and +100%, got {}%",
                        name,
                        p
                    ));
                }
            }
            Ok(())
        };

        check_offset("offset_x", &self.offset_x)?;
        check_offset("offset_y", &self.offset_y)?;

        Ok(())
    }
}
