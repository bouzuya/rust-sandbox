use crate::{client::new_client, event_id::event_id_from_hex_or_bech32};

// NIP-25 <https://github.com/nostr-protocol/nips/blob/master/25.md>
pub async fn handle(event_id: String) -> anyhow::Result<()> {
    let event_id = event_id_from_hex_or_bech32(event_id.as_str())?;
    let client = new_client().await?;
    client.dislike(event_id).await?;
    Ok(())
}
