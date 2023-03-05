use std::time::Duration;

use nostr_sdk::prelude::{Kind, SubscriptionFilter, ToBech32};
use time::format_description::well_known::Rfc3339;

use crate::{client::new_client, metadata_cache};

pub async fn list(me: bool) -> anyhow::Result<()> {
    let client = new_client().await?;

    let public_keys = if me {
        vec![client.keys().public_key()]
    } else {
        let contact_list = client.get_contact_list().await?;
        contact_list
            .into_iter()
            .map(|contact| contact.pk)
            .collect::<Vec<_>>()
    };

    let mut metadata_cache = metadata_cache::load()?;
    for public_key in public_keys.iter().copied() {
        if metadata_cache.get(public_key).is_none() {
            let metadata = client.get_metadata(public_key).await?;
            if let Some(metadata) = metadata.clone() {
                metadata_cache.set(public_key, metadata);
            }
        };
    }
    metadata_cache::store(&metadata_cache)?;

    let filter = SubscriptionFilter::new()
        .kind(Kind::TextNote)
        .authors(public_keys)
        .limit(32);
    let timeout = Duration::from_secs(10);
    let events = client.get_events_of(vec![filter], Some(timeout)).await?;
    for event in events.into_iter().rev() {
        println!(
            "@{} ({}) : ",
            metadata_cache
                .get(event.pubkey)
                .and_then(|m| m.name)
                .unwrap_or(event.pubkey.to_bech32()?),
            event.pubkey.to_bech32()?
        );
        println!("{}", event.content);
        println!(
            "{} {}",
            time::OffsetDateTime::from_unix_timestamp(event.created_at.as_i64())?
                .format(&Rfc3339)?,
            event.id.to_bech32()?
        );
        println!();
        // TODO: tags
    }
    Ok(())
}
