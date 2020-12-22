use anyhow::{ensure, Context, Result};
use dirs::cache_dir;
use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};

use crate::http_client::{HttpClient, HttpResponse};

pub fn list() -> Result<()> {
    let session_file = get_session_file()?;
    let cookie = read_to_string(session_file)?;
    let response = get_my_page_attendances(&cookie)?;
    println!("{:?}", response);
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

fn get_my_page_attendances(cookie: &str) -> Result<HttpResponse> {
    let url = "https://attendance.moneyforward.com/my_page/attendances";
    let client = HttpClient::new()?;
    let response = client.get(url, &[("Cookie", &cookie)])?;
    Ok(response)
}
