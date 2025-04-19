use crate::ImagePipeline;

impl ImagePipeline<'_> {
    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        self.load().await?;
        self.process()?;
        self.encode().await?;
        self.store().await?;
        Ok(())
    }

    pub async fn load(&mut self) -> Result<(), anyhow::Error> {
        self.image = Some(self.source.read(self.input).await?);
        if self.image.is_none() {
            return Err(anyhow::anyhow!("[core/pipeline] Image cannot be loaded"));
        }
        Ok(())
    }

    pub async fn store(&mut self) -> Result<(), anyhow::Error> {
        let encoded = self.encoded_bytes.as_ref().ok_or_else(|| {
            anyhow::anyhow!("[core/pipeline] No encoded image available for storage")
        })?;
        self.destination.write(self.output, encoded).await?;
        Ok(())
    }

    pub async fn encode(&mut self) -> Result<(), anyhow::Error> {
        let image = self
            .image
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No image loaded"))?;
        let encoder = self.resolve_encoder()?;
        let encoded = encoder.encode(image, &self.options)?;
        self.encoded_bytes = Some(encoded.clone());

        Ok(())
    }
}
