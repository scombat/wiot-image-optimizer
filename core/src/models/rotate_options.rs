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
            const EPSILON: f32 = 1e-6;
            if degrees.abs() < EPSILON {
                return Err(anyhow!("Rotate must be greater than 0.0"));
            }
        }
        Ok(())
    }
}
