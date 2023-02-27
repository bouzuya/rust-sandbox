use nostr_sdk::prelude::ToBech32;

use crate::client::new_client;

pub async fn handle(content: String) -> anyhow::Result<()> {
    let client = new_client().await?;
    let note_id = client.publish_text_note(content, &[]).await?.to_bech32()?;
    println!("{note_id}");
    Ok(())
}
