use crate::ImagePipeline;

impl ImagePipeline<'_> {
    pub fn process(&mut self) -> Result<(), anyhow::Error> {
        self.crop()?;
        self.resize()?;
        self.optimize_quality()?;
        Ok(())
    }
}
