use crate::ImagePipeline;

impl ImagePipeline<'_> {
    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        self.load().await?;
        self.process()?;

        match self.options.auto_select_format {
            true => self.encode_auto_format().await?,
            _ => self.encode().await?,
        }

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
        self.destination.write(&self.output, encoded).await?;
        Ok(())
    }

    pub async fn encode(&mut self) -> Result<(), anyhow::Error> {
        let encoder = self.resolve_encoder()?;

        // Flatten alpha if encoder doesn't support transparency and image has alpha
        let image = if !encoder.supports_transparency() && self.image_has_alpha() {
            self.flatten_alpha()?
        } else {
            self.image
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("No image loaded"))?
                .clone()
        };

        let encoded = encoder.encode(&image, &self.options)?;
        self.encoded_bytes = Some(encoded);

        Ok(())
    }
}
