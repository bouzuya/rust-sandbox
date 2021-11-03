use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    data_dir: PathBuf,
    time_zone_offset: String,
}

impl Config {
    pub fn new(data_dir: PathBuf, time_zone_offset: String) -> Self {
        Self {
            data_dir,
            time_zone_offset,
        }
    }

    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }

    pub fn time_zone_offset(&self) -> &str {
        self.time_zone_offset.as_str()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigJson {
    data_dir: PathBuf,
    time_zone_offset: String,
}

impl From<ConfigJson> for Config {
    fn from(config_json: ConfigJson) -> Self {
        Self {
            data_dir: config_json.data_dir,
            time_zone_offset: config_json.time_zone_offset,
        }
    }
}

impl From<Config> for ConfigJson {
    fn from(config: Config) -> Self {
        Self {
            data_dir: config.data_dir,
            time_zone_offset: config.time_zone_offset,
        }
    }
}

#[derive(Debug)]
pub struct ConfigRepository;

impl ConfigRepository {
    pub fn new() -> Self {
        Self
    }

    pub fn load(&self) -> anyhow::Result<Config> {
        let config_file = ConfigRepository::config_file()?;
        let content = fs::read_to_string(config_file.as_path())?;
        let config_json = serde_json::from_str::<'_, ConfigJson>(content.as_str())?;
        let config = Config::from(config_json);
        Ok(config)
    }

    pub fn save(&self, config: Config) -> anyhow::Result<()> {
        let config_file = ConfigRepository::config_file()?;
        let parent = config_file.parent().context("no config_dir")?;
        fs::create_dir_all(parent)?;
        let config_json = ConfigJson::from(config);
        fs::write(config_file, serde_json::to_string(&config_json)?)?;
        Ok(())
    }

    fn config_file() -> anyhow::Result<PathBuf> {
        let config_dir = ConfigRepository::config_dir()?;
        let config_file = config_dir.join("config.json");
        Ok(config_file)
    }

    fn config_dir() -> anyhow::Result<PathBuf> {
        let prefix = "net.bouzuya.rust-sandbox.b";
        Ok(match env::var_os("B_TEST_CONFIG_DIR") {
            Some(test_config_dir) => PathBuf::from(test_config_dir),
            None => BaseDirectories::with_prefix(prefix)?.get_config_home(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn struct_test() -> anyhow::Result<()> {
        let data_dir = PathBuf::from_str("/path/to/data")?;
        let time_zone_offset = "+09:00".to_string();
        let config = Config::new(data_dir.clone(), time_zone_offset);
        assert_eq!(config.data_dir(), data_dir.as_path());
        assert_eq!(config.time_zone_offset(), "+09:00");
        assert_eq!(config.clone(), config);
        Ok(())
    }

    #[test]
    fn repository_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        // config
        let data_dir = temp_dir.path().join("data");
        fs::create_dir_all(data_dir.as_path())?;
        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(config_dir.as_path())?;
        let config_file = config_dir.join("config.json");
        env::set_var(
            "B_TEST_CONFIG_DIR",
            config_dir.to_str().context("config dir is not UTF-8")?,
        );

        let time_zone_offset = "+09:00";
        let config = Config::new(data_dir.clone(), time_zone_offset.to_string());
        let repository = ConfigRepository::new();
        repository.save(config.clone())?;
        let loaded = repository.load()?;
        assert_eq!(loaded, config);

        let saved = fs::read_to_string(config_file.as_path())?;
        assert_eq!(
            saved,
            format!(
                r#"{{"data_dir":"{}","time_zone_offset":"{}"}}"#,
                data_dir.to_str().context("data_dir.to_str()")?,
                time_zone_offset,
            )
        );

        Ok(())
    }
}
