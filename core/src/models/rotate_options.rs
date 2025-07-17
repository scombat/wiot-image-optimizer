use anyhow::{Result, anyhow};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RotateOptions {
    pub angle_degrees: Option<f32>,
}

impl RotateOptions {
    pub fn is_enabled(&self) -> bool {
        self.angle_degrees.is_some()
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if let Some(degrees) = self.angle_degrees {
            if degrees == 0.0 {
                return Err(anyhow!("Rotate must be between greater than 0.0"));
            }
        }
        Ok(())
    }
}
