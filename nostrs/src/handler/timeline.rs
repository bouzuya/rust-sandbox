use std::time::Duration;

use nostr_sdk::prelude::{Kind, SubscriptionFilter, Timestamp};

use crate::client::new_client;

pub async fn handle() -> anyhow::Result<()> {
    // TODO: show timeline
    let client = new_client().await?;
    let subscription = SubscriptionFilter::new()
        .kind(Kind::TextNote)
        .author(client.keys().public_key())
        .since(Timestamp::now() - Duration::from_secs(60 * 60));
    let timeout = Duration::from_secs(10);
    let events = client
        .get_events_of(vec![subscription], Some(timeout))
        .await?;
    for event in events {
        println!("{event:?}");
    }
    Ok(())
}
