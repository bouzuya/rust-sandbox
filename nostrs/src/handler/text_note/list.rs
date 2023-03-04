use std::time::Duration;

use nostr_sdk::prelude::{Kind, SubscriptionFilter, ToBech32};
use time::format_description::well_known::Rfc3339;

use crate::client::new_client;

pub async fn handle() -> anyhow::Result<()> {
    let client = new_client().await?;
    let filter = SubscriptionFilter::new()
        .kind(Kind::TextNote)
        .author(client.keys().public_key())
        .limit(32);
    let timeout = Duration::from_secs(10);
    let events = client.get_events_of(vec![filter], Some(timeout)).await?;
    for event in events.into_iter().rev() {
        println!("{}", event.id.to_bech32()?);
        println!("{} : ", event.pubkey);
        println!(
            "{} ({})",
            event.content,
            time::OffsetDateTime::from_unix_timestamp(event.created_at.as_i64())?
                .format(&Rfc3339)?
        );
        println!();
        // TODO: tags
    }

    Ok(())
}
