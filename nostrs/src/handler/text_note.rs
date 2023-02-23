use std::{env, time::Duration};

use nostr_sdk::{
    prelude::{FromSkStr, Keys, Kind, SubscriptionFilter},
    Client,
};

pub async fn handle() -> anyhow::Result<()> {
    let my_keys = Keys::from_sk_str(env::var("SECRET_KEY")?.as_str())?;

    let client = Client::new(&my_keys);
    client
        .add_relay("wss://nostr-pub.wellorder.net", None)
        .await?;
    client.connect().await;

    let filter = SubscriptionFilter::new()
        .kind(Kind::TextNote)
        .author(my_keys.public_key())
        .limit(32);
    let timeout = Duration::from_secs(10);
    let events = client.get_events_of(vec![filter], Some(timeout)).await?;
    for event in events {
        println!("{}", serde_json::to_string_pretty(&event)?);
    }
    Ok(())
}
