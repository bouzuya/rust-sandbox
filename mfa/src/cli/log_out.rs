use anyhow::{ensure, Context, Result};
use dirs::cache_dir;
use std::{
    fs::{create_dir_all, remove_file},
    path::PathBuf,
};

pub fn log_out() -> Result<()> {
    let session_file = get_session_file()?;
    if session_file.exists() {
        remove_file(&session_file)?;
        println!(
            "session file removed: {}",
            session_file
                .to_str()
                .with_context(|| "session_file.to_str()")?
        );
    }
    Ok(())
}

fn get_session_file() -> Result<PathBuf> {
    let cache_dir = cache_dir().with_context(|| "dirs::cache_dir")?;
    let app_cache_dir = cache_dir.join("rust-sandbox-mfa");
    if !app_cache_dir.is_dir() {
        ensure!(!app_cache_dir.exists(), "cache_dir is not dir");
        create_dir_all(&app_cache_dir).with_context(|| "fs::create_dir_all(cache_dir)")?;
    }
    Ok(app_cache_dir.join("session"))
}
