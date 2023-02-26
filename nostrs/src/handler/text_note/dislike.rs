use std::time::Duration;

use anyhow::bail;
use nostr_sdk::prelude::{EventId, Kind, SubscriptionFilter};

use crate::client::new_client;

// NIP-25 <https://github.com/nostr-protocol/nips/blob/master/25.md>
pub async fn handle(event_id: String) -> anyhow::Result<()> {
    let event_id = EventId::from_hex(event_id)?;
    let filter = SubscriptionFilter::new()
        .kind(Kind::TextNote)
        .id(event_id.to_hex())
        .limit(1);
    let timeout = Duration::from_secs(10);
    let client = new_client().await?;
    let events = client.get_events_of(vec![filter], Some(timeout)).await?;
    if events.is_empty() {
        bail!("event ({event_id:?}) not found");
    }
    let public_key = events[0].pubkey;
    client.dislike(event_id, public_key).await?;
    Ok(())
}
