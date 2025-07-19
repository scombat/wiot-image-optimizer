use crate::ImagePipeline;

impl ImagePipeline<'_> {
    pub fn process(&mut self) -> Result<(), anyhow::Error> {
        self.image_adjustments()?;
        self.rotate()?;
        self.crop()?;
        self.blur()?;
        self.resize()?;
        self.optimize_quality()?;
        self.mirror()?;
        Ok(())
    }
}
