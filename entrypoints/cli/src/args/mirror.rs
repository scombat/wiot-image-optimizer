use clap::Args;
use wiot_core::models::options::MirrorOptions;

#[derive(Args, Debug)]
pub struct MirrorArgs {
    // Mirror image left-to-right
    #[arg(long, help = "Flip (horizontal mirror)")]
    pub flip: bool,

    // Mirror image top-to-bottom
    #[arg(long, help = "Flop (vertical mirror)")]
    pub flop: bool,
}

impl MirrorArgs {
    pub fn get(&self) -> Option<MirrorOptions> {
        if !self.flip && !self.flop {
            return None;
        }

        Some(MirrorOptions {
            flip: self.flip,
            flop: self.flop,
        })
    }
}
