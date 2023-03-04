use std::{collections::HashMap, time::Duration};

use nostr_sdk::prelude::{Kind, SubscriptionFilter, Timestamp, ToBech32};
use time::format_description::well_known::Rfc3339;

use crate::{
    client::new_client,
    contact::{self, Contacts},
};

pub async fn handle(me: bool) -> anyhow::Result<()> {
    let client = new_client().await?;
    let (public_keys, metadata_map) = if me {
        let public_keys = vec![client.keys().public_key()];
        let mut metadata_map = HashMap::new();
        let metadata = client.get_metadata(client.keys().public_key()).await?;
        metadata_map.insert(client.keys().public_key(), metadata);
        (public_keys, metadata_map)
    } else {
        let contact_list = client.get_contact_list().await?;

        // get contacts
        let contact_cache = contact::load()?;
        let now = Timestamp::now();
        let metadata_map = match contact_cache.updated_at {
            Some(t) if t >= now - Duration::from_secs(60 * 60) => contact_cache.contacts,
            Some(_) | None => {
                let mut map = HashMap::new();
                for contact in contact_list.iter() {
                    map.insert(contact.pk, client.get_metadata(contact.pk).await?);
                }
                contact::store(&Contacts {
                    contacts: map.clone(),
                    updated_at: Some(now),
                })?;

                map
            }
        };

        (
            contact_list.into_iter().map(|contact| contact.pk).collect(),
            metadata_map,
        )
    };

    let filter = SubscriptionFilter::new()
        .kind(Kind::TextNote)
        .authors(public_keys)
        .limit(32);
    let timeout = Duration::from_secs(10);
    let events = client.get_events_of(vec![filter], Some(timeout)).await?;
    for event in events.into_iter().rev() {
        println!(
            "@{} ({}) : ",
            metadata_map
                .get(&event.pubkey)
                .cloned()
                .and_then(|m| m.and_then(|metadata| metadata.name))
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
