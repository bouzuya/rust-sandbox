use anyhow::Context;

use crate::client::Client;

pub async fn get() -> anyhow::Result<()> {
    let client = Client::new().await?;
    let metadata = client
        .get_metadata(client.keys().public_key(), None)
        .await?
        .context("metadata not found")?;
    println!("{}", serde_json::to_string_pretty(&metadata)?);
    Ok(())
}
