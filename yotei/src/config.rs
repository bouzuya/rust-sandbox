use anyhow::Context as _;

#[derive(serde::Deserialize)]
pub struct Config {
    pub calendar_id: String,
    pub debug: bool,
    // env GOOGLE_APPLICATION_CREDENTIALS
    pub impersonate_user_email: Option<String>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("net.bouzuya.rust-sandbox.yotei")?;
        let config_file_path = xdg_dirs
            .place_config_file("config.json")
            .context("The parent directory of the config file could not be created")?;
        let config_file_content =
            std::fs::read_to_string(&config_file_path).with_context(|| {
                format!("The config file could not be read ({:?})", config_file_path)
            })?;
        let config = serde_json::from_str::<Config>(&config_file_content)
            .context("The config file could not be parsed")?;
        Ok(config)
    }
}
