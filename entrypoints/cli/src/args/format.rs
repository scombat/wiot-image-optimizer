use clap::Args;

#[derive(Args, Debug)]
pub struct FormatArgs {
    /// Output image format (jpeg, png, webp, avif, gif)
    ///
    /// If not specified, the format will be inferred from the output file extension,
    /// unless --auto-format is enabled.
    #[arg(long, value_name = "FORMAT", conflicts_with = "auto_format")]
    pub format: Option<String>,

    /// Enable automatic selection of the output format.
    ///
    /// The format will be chosen to produce the smallest image.
    /// Default formats are: WebP, Avif, and Jpeg or Png depending on whether the image contains transparency.
    /// This flag is incompatible with the "format" option.
    #[arg(long, conflicts_with_all = &["format"])]
    pub auto_format: bool,
}
