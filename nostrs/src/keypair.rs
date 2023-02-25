use std::{
    fs::{self, File},
    io::BufReader,
};

use crate::dirs::state_dir;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Json {
    private_key: String,
}

pub fn load() -> anyhow::Result<String> {
    let path = state_dir()?.join("private_key.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let json: Json = serde_json::from_reader(reader)?;
    Ok(json.private_key)
}

pub fn store(private_key: String) -> anyhow::Result<()> {
    let path = state_dir()?.join("private_key.json");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string(&Json { private_key })?;
    fs::write(path, content)?;
    Ok(())
}
