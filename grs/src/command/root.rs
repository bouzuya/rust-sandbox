use std::{env, path::PathBuf};

pub fn root() -> anyhow::Result<PathBuf> {
    Ok(match env::var("GRS_ROOT") {
        Ok(s) => PathBuf::from(s),
        Err(_) => {
            let home = env::var("HOME")?;
            PathBuf::from(home).join("grs")
        }
    })
}
