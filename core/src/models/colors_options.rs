#[derive(Debug, Default, Clone)]
pub struct ColorsOptions {
    pub grayscale: bool,
    pub brightness: Option<i32>,
    pub contrast: Option<f32>,
    pub gamma: Option<f32>,
    pub invert: bool,
}

impl ColorsOptions {
    pub fn is_enabled(&self) -> bool {
        self.grayscale
            || self.brightness.is_some()
            || self.contrast.is_some()
            || self.gamma.is_some()
            || self.invert
    }
}
