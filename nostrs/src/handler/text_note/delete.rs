use crate::{client::Client, event_id::event_id_from_hex_or_bech32};

// NIP-09 <https://github.com/nostr-protocol/nips/blob/master/09.md>
pub async fn delete(event_id: String) -> anyhow::Result<()> {
    let event_id = event_id_from_hex_or_bech32(event_id.as_str())?;
    let client = Client::new().await?;
    client.delete_event(event_id).await?;
    Ok(())
}
