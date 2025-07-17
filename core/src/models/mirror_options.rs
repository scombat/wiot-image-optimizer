#[derive(Debug, Default, Clone)]
pub struct MirrorOptions {
    pub flip: bool,
    pub flop: bool,
}

impl MirrorOptions {
    pub fn is_enabled(&self) -> bool {
        self.flip || self.flop
    }
}
