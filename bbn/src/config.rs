use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    data_dir: PathBuf,
    hatena_blog_data_file: PathBuf,
}

impl Config {
    pub fn new(data_dir: PathBuf, hatena_blog_data_file: PathBuf) -> Self {
        Self {
            data_dir,
            hatena_blog_data_file,
        }
    }

    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }

    pub fn hatena_blog_data_file(&self) -> &Path {
        self.hatena_blog_data_file.as_path()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn config_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().join("data");
        fs::create_dir_all(data_dir.as_path())?;
        let hatena_blog_data_file = temp_dir.path().join("hatena_blog.db");
        let config = Config::new(data_dir.clone(), hatena_blog_data_file.clone());
        assert_eq!(config.data_dir(), data_dir.as_path());
        assert_eq!(
            config.hatena_blog_data_file(),
            hatena_blog_data_file.as_path()
        );
        assert_eq!(config.clone(), config);
        Ok(())
    }
}
