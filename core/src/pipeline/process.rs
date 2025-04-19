use crate::ImagePipeline;

impl ImagePipeline<'_> {
    pub fn process(&mut self) -> Result<(), anyhow::Error> {
        self.resize()?;
        self.optimize_quality()?;
        Ok(())
    }
}
