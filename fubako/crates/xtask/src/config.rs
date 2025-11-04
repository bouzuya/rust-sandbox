pub(crate) struct Config {
    pub(crate) data_dir: std::path::PathBuf,
}

impl Config {
    pub(crate) async fn load() -> anyhow::Result<Config> {
        Ok(Config {
            data_dir: std::path::PathBuf::from("data"),
        })
    }
}
