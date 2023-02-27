use crate::{client::new_client, event_id::event_id_from_bech32_or_hex};

// NIP-09 <https://github.com/nostr-protocol/nips/blob/master/09.md>
pub async fn handle(event_id: String) -> anyhow::Result<()> {
    let event_id = event_id_from_bech32_or_hex(event_id.as_str())?;
    let client = new_client().await?;
    client.delete_event::<String>(event_id, None).await?;
    Ok(())
}
