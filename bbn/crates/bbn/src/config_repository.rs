use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

use crate::{config::Config, credentials::Credentials};

#[derive(Debug, Deserialize, Serialize)]
struct ConfigJson {
    data_dir: PathBuf,
    hatena_blog_data_file: PathBuf,
    link_completion_rules_file: Option<PathBuf>,
}

impl From<ConfigJson> for Config {
    fn from(config_json: ConfigJson) -> Self {
        Self::new(
            config_json.data_dir,
            config_json.hatena_blog_data_file,
            config_json.link_completion_rules_file,
        )
    }
}

impl From<Config> for ConfigJson {
    fn from(config: Config) -> Self {
        Self {
            data_dir: config.data_dir().to_path_buf(),
            hatena_blog_data_file: config.hatena_blog_data_file().to_path_buf(),
            link_completion_rules_file: config
                .link_completion_rules_file()
                .map(|it| it.to_path_buf()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct CredentialsJson {
    hatena_api_key: String,
    hatena_blog_id: String,
    hatena_id: String,
}

impl From<CredentialsJson> for Credentials {
    fn from(credentials_json: CredentialsJson) -> Self {
        Self::new(
            credentials_json.hatena_api_key,
            credentials_json.hatena_blog_id,
            credentials_json.hatena_id,
        )
    }
}

impl From<Credentials> for CredentialsJson {
    fn from(credentials: Credentials) -> Self {
        Self {
            hatena_api_key: credentials.hatena_api_key().to_string(),
            hatena_blog_id: credentials.hatena_blog_id().to_string(),
            hatena_id: credentials.hatena_id().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ConfigRepository(PathBuf);

impl ConfigRepository {
    pub fn new() -> anyhow::Result<Self> {
        let prefix = "net.bouzuya.rust-sandbox.bbn";
        Ok(Self(match env::var_os("BBN_TEST_CONFIG_DIR") {
            Some(test_config_dir) => PathBuf::from(test_config_dir),
            None => BaseDirectories::with_prefix(prefix)?.get_config_home(),
        }))
    }

    pub fn load(&self) -> anyhow::Result<Config> {
        let config_file = self.config_file()?;
        let content = fs::read_to_string(config_file.as_path())?;
        let config_json = serde_json::from_str::<'_, ConfigJson>(content.as_str())?;
        let config = Config::from(config_json);
        Ok(config)
    }

    pub fn load_credentials(&self) -> anyhow::Result<Credentials> {
        let credential_file = self.credential_file()?;
        let content = fs::read_to_string(credential_file.as_path())?;
        let credentials_json = serde_json::from_str::<'_, CredentialsJson>(content.as_str())?;
        let credentials = Credentials::from(credentials_json);
        Ok(credentials)
    }

    // NOTE: The repository exposes its dependency on fs.
    pub fn credential_file_path(&self) -> anyhow::Result<PathBuf> {
        self.credential_file()
    }

    // NOTE: The repository exposes its dependency on fs.
    pub fn path(&self) -> anyhow::Result<PathBuf> {
        self.config_file()
    }

    pub fn save(&self, config: Config) -> anyhow::Result<()> {
        let config_file = self.config_file()?;
        let parent = config_file.parent().context("no config_dir")?;
        fs::create_dir_all(parent)?;
        let config_json = ConfigJson::from(config);
        fs::write(config_file, serde_json::to_string(&config_json)?)?;
        Ok(())
    }

    fn config_dir(&self) -> &Path {
        self.0.as_path()
    }

    fn config_file(&self) -> anyhow::Result<PathBuf> {
        let config_dir = self.config_dir();
        let config_file = config_dir.join("config.json");
        Ok(config_file)
    }

    fn credential_file(&self) -> anyhow::Result<PathBuf> {
        let config_dir = self.config_dir();
        let credential_file = config_dir.join("credentials.json");
        Ok(credential_file)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    #[test]
    fn load_credentials_test() -> anyhow::Result<()> {
        let hatena_api_key = "hatena_api_key1";
        let hatena_blog_id = "hatena_blog_id1";
        let hatena_id = "hatena_id1";

        let temp_dir = tempdir()?;
        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(config_dir.as_path())?;
        let credential_file = config_dir.join("credentials.json");
        fs::write(
            credential_file.as_path(),
            format!(
                r#"{{"hatena_api_key":"{hatena_api_key}","hatena_blog_id":"{hatena_blog_id}","hatena_id":"{hatena_id}"}}"#
            ),
        )?;
        env::set_var(
            "BBN_TEST_CONFIG_DIR",
            config_dir.to_str().context("config dir is not UTF-8")?,
        );

        let credentials = Credentials::new(
            hatena_api_key.to_string(),
            hatena_blog_id.to_string(),
            hatena_id.to_string(),
        );

        let repository = ConfigRepository::new()?;
        let loaded = repository.load_credentials()?;
        assert_eq!(loaded, credentials);

        assert_eq!(repository.credential_file_path()?, credential_file);
        Ok(())
    }
    use super::*;

    #[test]
    fn repository_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        // config
        let data_dir = temp_dir.path().join("data");
        fs::create_dir_all(data_dir.as_path())?;
        let hatena_blog_data_file = temp_dir.path().join("hatena_blog.db");
        let link_completion_rules_file = temp_dir.path().join("link_completion_rules.json");
        // config_repository
        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(config_dir.as_path())?;
        let config_file = config_dir.join("config.json");
        env::set_var(
            "BBN_TEST_CONFIG_DIR",
            config_dir.to_str().context("config dir is not UTF-8")?,
        );

        let config = Config::new(
            data_dir.clone(),
            hatena_blog_data_file.clone(),
            Some(link_completion_rules_file.clone()),
        );
        let repository = ConfigRepository::new()?;
        repository.save(config.clone())?;
        let loaded = repository.load()?;
        assert_eq!(loaded, config);

        let saved = fs::read_to_string(config_file.as_path())?;
        assert_eq!(
            saved,
            format!(
                r#"{{"data_dir":"{}","hatena_blog_data_file":"{}","link_completion_rules_file":"{}"}}"#,
                data_dir.to_str().context("data_dir.to_str()")?,
                hatena_blog_data_file
                    .to_str()
                    .context("hatena_blog_data_file.to_str()")?,
                link_completion_rules_file
                    .to_str()
                    .context("link_completion_rules_file.to_str()")?
            )
        );

        assert_eq!(repository.path()?, config_file);
        Ok(())
    }
}
