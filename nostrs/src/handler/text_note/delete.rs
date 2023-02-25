use nostr_sdk::prelude::EventId;

use crate::client::new_client;

// NIP-09 <https://github.com/nostr-protocol/nips/blob/master/09.md>
pub async fn handle(event_id: String) -> anyhow::Result<()> {
    let event_id = EventId::from_hex(event_id)?;
    let client = new_client().await?;
    client.delete_event::<String>(event_id, None).await?;
    Ok(())
}
