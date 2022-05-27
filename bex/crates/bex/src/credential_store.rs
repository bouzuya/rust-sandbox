use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::store::Store;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credential {
    pub access_token: String,
    pub consumer_key: String,
    pub username: String,
}

impl Credential {
    pub(crate) fn new(access_token: String, consumer_key: String, username: String) -> Credential {
        Self {
            access_token,
            consumer_key,
            username,
        }
    }
}

pub struct CredentialStore {
    path: PathBuf,
}

impl CredentialStore {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            path: dir.as_ref().join("credential.json"),
        }
    }
}

impl Store for CredentialStore {
    type Item = Credential;

    fn path(&self) -> &Path {
        self.path.as_path()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO
    }
}
