use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Credential {
    pub access_token: String,
    pub username: String,
}

impl Credential {
    pub(crate) fn new(access_token: String, username: String) -> Credential {
        Self {
            access_token,
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

    pub fn load(&self) -> anyhow::Result<Option<Credential>> {
        let p = self.path.as_path();
        if p.exists() {
            let s = fs::read_to_string(p)?;
            Ok(serde_json::from_str(&s)?)
        } else {
            Ok(None)
        }
    }

    pub fn store(&self, credential: &Credential) -> anyhow::Result<()> {
        let p = self.path.as_path();
        if let Some(dir) = p.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(p, serde_json::to_string(credential)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO
    }
}
