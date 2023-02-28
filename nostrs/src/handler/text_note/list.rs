use std::{cmp::Reverse, collections::HashSet, time::Duration};

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
    let mut unique_events = vec![];
    let mut used = HashSet::new();
    for event in events {
        if used.insert(event.id) {
            unique_events.push(event);
        }
    }
    unique_events.sort_by_key(|event| Reverse(event.created_at));

    for event in unique_events {
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
