use std::time::Duration;

use nostr_sdk::prelude::{Kind, SubscriptionFilter};

use crate::client::new_client;

pub async fn handle() -> anyhow::Result<()> {
    let client = new_client().await?;
    let filter = SubscriptionFilter::new()
        .kind(Kind::TextNote)
        .author(client.keys().public_key())
        .limit(32);
    let timeout = Duration::from_secs(10);
    let events = client.get_events_of(vec![filter], Some(timeout)).await?;
    for event in events {
        println!("{}", serde_json::to_string_pretty(&event)?);
    }
    Ok(())
}
