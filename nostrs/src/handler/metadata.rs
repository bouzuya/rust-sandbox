use std::{env, time::Duration};

use anyhow::Context;
use nostr_sdk::{
    prelude::{FromSkStr, Keys, Kind, SubscriptionFilter},
    Client,
};
use serde_json::Value;

pub async fn handle() -> anyhow::Result<()> {
    let my_keys = Keys::from_sk_str(env::var("SECRET_KEY")?.as_str())?;

    let client = Client::new(&my_keys);
    client
        .add_relay("wss://nostr-pub.wellorder.net", None)
        .await?;
    client.connect().await;

    let filter = SubscriptionFilter::new()
        .kind(Kind::Metadata)
        .author(my_keys.public_key())
        .limit(1);
    let timeout = Duration::from_secs(10);
    let events = client.get_events_of(vec![filter], Some(timeout)).await?;
    let event = events.first().context("metadata not found")?;
    let metadata: Value = serde_json::from_str(event.content.as_str())?;
    println!("{}", serde_json::to_string_pretty(&metadata)?);
    Ok(())
}
