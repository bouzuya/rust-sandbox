mod image;
mod new;
mod preview;

#[derive(clap::Subcommand)]
pub(crate) enum Subcommand {
    /// Manage images
    #[clap(subcommand)]
    Image(crate::subcommand::image::ImageCommand),
    /// Create a new page
    New,
    /// Start a local preview server
    Preview,
}

impl Subcommand {
    pub(crate) async fn execute(self) -> anyhow::Result<()> {
        match self {
            Subcommand::Image(image_command) => self::image::execute(image_command).await,
            Subcommand::New => self::new::execute().await,
            Subcommand::Preview => self::preview::execute().await,
        }
    }
}
