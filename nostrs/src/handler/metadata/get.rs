use std::time::Duration;

use anyhow::Context;
use nostr_sdk::prelude::{Kind, Metadata, SubscriptionFilter};

use crate::client::new_client;

pub async fn handle() -> anyhow::Result<()> {
    let client = new_client().await?;

    let filter = SubscriptionFilter::new()
        .kind(Kind::Metadata)
        .author(client.keys().public_key())
        .limit(1);
    let timeout = Duration::from_secs(10);
    let events = client.get_events_of(vec![filter], Some(timeout)).await?;
    let event = events.first().context("metadata not found")?;
    let metadata: Metadata = serde_json::from_str(event.content.as_str())?;
    println!("{}", serde_json::to_string_pretty(&metadata)?);
    Ok(())
}
