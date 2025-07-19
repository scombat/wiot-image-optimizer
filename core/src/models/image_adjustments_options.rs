#[derive(Debug, Default, Clone)]
pub struct Sharpen {
    pub sigma: f32,
    pub threshold: i32,
}

#[derive(Debug, Default, Clone)]
pub struct ImageAdjustmentsOptions {
    pub grayscale: bool,
    pub brightness: Option<i32>,
    pub contrast: Option<f32>,
    pub gamma: Option<f32>,
    pub invert: bool,
    pub sharpen: Option<Sharpen>,
}

impl ImageAdjustmentsOptions {
    pub fn is_enabled(&self) -> bool {
        self.grayscale
            || self.brightness.is_some()
            || self.contrast.is_some()
            || self.gamma.is_some()
            || self.invert
            || self.sharpen.is_some()
    }
}
