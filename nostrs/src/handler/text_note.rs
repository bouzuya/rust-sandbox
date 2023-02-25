use std::{collections::HashSet, time::Duration};

use anyhow::bail;
use nostr_sdk::prelude::{EventId, Kind, SubscriptionFilter};

use crate::client::new_client;

pub async fn create(content: String) -> anyhow::Result<()> {
    let client = new_client().await?;
    let event_id = client.publish_text_note(content, &[]).await?;
    println!("{event_id:?}");
    Ok(())
}

// NIP-09 <https://github.com/nostr-protocol/nips/blob/master/09.md>
pub async fn delete(event_id: String) -> anyhow::Result<()> {
    let event_id = EventId::from_hex(event_id)?;
    let client = new_client().await?;
    client.delete_event::<String>(event_id, None).await?;
    Ok(())
}

// NIP-25 <https://github.com/nostr-protocol/nips/blob/master/25.md>
pub async fn like(event_id: String) -> anyhow::Result<()> {
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
    client.like(event_id, public_key).await?;
    Ok(())
}

pub async fn list() -> anyhow::Result<()> {
    let client = new_client().await?;
    let filter = SubscriptionFilter::new()
        .kind(Kind::TextNote)
        .author(client.keys().public_key())
        .limit(32);
    let timeout = Duration::from_secs(10);
    let events = client.get_events_of(vec![filter], Some(timeout)).await?;
    let mut used = HashSet::new();
    for event in events {
        if used.insert(event.id) {
            println!("{}", serde_json::to_string_pretty(&event)?);
        }
    }
    Ok(())
}
