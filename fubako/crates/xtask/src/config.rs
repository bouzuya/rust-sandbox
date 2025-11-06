#[derive(serde::Deserialize)]
pub(crate) struct Config {
    pub(crate) data_dir: std::path::PathBuf,
    pub(crate) image_bucket_name: String,
    pub(crate) image_object_prefix: String,
}

impl Config {
    pub(crate) async fn load() -> anyhow::Result<Config> {
        // TODO: config path
        let content = std::fs::read_to_string("config.json")?;
        Ok(serde_json::from_str(&content)?)
    }
}
