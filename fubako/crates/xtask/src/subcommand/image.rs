use anyhow::Context;

#[derive(clap::Subcommand)]
pub(crate) enum ImageCommand {
    /// Upload an image
    Upload { file_name: std::path::PathBuf },
}

pub(super) async fn execute(command: ImageCommand) -> anyhow::Result<()> {
    match command {
        ImageCommand::Upload { file_name } => {
            let config = crate::config::Config::load().await?;
            let image_bucket_name = config.image_bucket_name.clone();
            let image_object_prefix = config.image_object_prefix.clone();
            let images_dir = config.data_dir.join("images").canonicalize()?;
            let image_path = images_dir.join(file_name).canonicalize()?;
            if !image_path.starts_with(images_dir) {
                return Err(anyhow::anyhow!("Invalid image path"));
            }

            let client = google_cloud_storage::client::Storage::builder()
                .build()
                .await?;
            let object_name = image_path
                .file_name()
                .context("file_name")?
                .to_str()
                .context("file_name is not UTF-8")?;
            let payload = tokio::fs::File::open(&image_path).await?;
            let object = client
                .write_object(
                    format!("projects/_/buckets/{image_bucket_name}"),
                    format!("{image_object_prefix}{object_name}"),
                    payload,
                )
                .send_buffered()
                .await?;
            println!("Uploaded image to: {}/{}", object.bucket, object.name);
        }
    }
    Ok(())
}
