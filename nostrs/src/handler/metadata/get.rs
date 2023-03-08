use anyhow::Context;

use crate::client::new_client;

pub async fn get() -> anyhow::Result<()> {
    let client = new_client().await?;
    let metadata = client
        .get_metadata(client.keys().public_key(), None)
        .await?
        .context("metadata not found")?;
    println!("{}", serde_json::to_string_pretty(&metadata)?);
    Ok(())
}
