use std::{collections::HashSet, time::Duration};

use nostr_sdk::prelude::{Kind, SubscriptionFilter};

use crate::client::new_client;

pub async fn create(content: String) -> anyhow::Result<()> {
    let client = new_client().await?;
    let event_id = client.publish_text_note(content, &[]).await?;
    println!("{event_id:?}");
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
