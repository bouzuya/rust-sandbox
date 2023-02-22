use std::{env, time::Duration};

use nostr_sdk::{
    prelude::{FromSkStr, Keys, Kind, SubscriptionFilter, Timestamp, ToBech32},
    Client,
};

pub async fn handle() -> anyhow::Result<()> {
    // TODO: show timeline

    let my_keys = Keys::from_sk_str(env::var("SECRET_KEY")?.as_str())?;
    let bech32_pubkey = my_keys.public_key().to_bech32()?;
    println!("Bech32 PubKey: {}", bech32_pubkey);

    let client = Client::new(&my_keys);

    client.add_relay("wss://relay.damus.io", None).await?;
    client.connect().await;

    let subscription = SubscriptionFilter::new()
        .kind(Kind::TextNote)
        .author(my_keys.public_key())
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
