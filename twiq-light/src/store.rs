use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct TweetStore {
    path: PathBuf,
}

impl Default for TweetStore {
    fn default() -> Self {
        let path = Path::new(&env::var("HOME").expect("env HOME")).join("twiq-light.json");
        Self { path }
    }
}

impl TweetStore {
    pub fn path(&self) -> &Path {
        &self.path
    }
}
