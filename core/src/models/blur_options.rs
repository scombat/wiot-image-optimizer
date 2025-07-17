use anyhow::Result;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlurOptions {
    pub blur: Option<f32>,
}

impl BlurOptions {
    pub fn is_enabled(&self) -> bool {
        self.blur.is_some()
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if let Some(blur) = self.blur {
            if blur <= 0.0 {
                return Err(anyhow::anyhow!("Blur value must be greater than 0.0"));
            }
        }
        Ok(())
    }
}
