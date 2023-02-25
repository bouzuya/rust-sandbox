use std::{collections::HashMap, time::Duration};

use anyhow::bail;
use nostr_sdk::prelude::{Contact, Kind, Metadata, SubscriptionFilter, Tag, Timestamp};

use crate::{
    client::new_client,
    contact::{self, Contacts},
};

pub async fn handle() -> anyhow::Result<()> {
    let client = new_client().await?;

    let contact_cache = contact::load()?;
    let now = Timestamp::now();
    let contact_list = match contact_cache.updated_at {
        Some(t) if t >= now - Duration::from_secs(60 * 60) => contact_cache.contacts,
        Some(_) | None => {
            let mut map = HashMap::new();
            let filter = SubscriptionFilter::new()
                .authors(vec![client.keys().public_key()])
                .kind(Kind::ContactList)
                .limit(1);
            let timeout = Duration::from_secs(10);
            let events = client.get_events_of(vec![filter], Some(timeout)).await?;

            let mut contact_list = HashMap::new();
            let mut contact_list_timestamp = None;
            for event in events {
                let mut list = HashMap::new();
                for tag in event.tags {
                    let contact = match tag {
                        Tag::ContactList {
                            pk,
                            relay_url,
                            alias,
                        } => Contact::new(pk, relay_url, alias),
                        Tag::PubKey(pk, _) => Contact::new::<String>(pk, None, None),
                        _ => bail!("invalid tag: {tag:?}"),
                    };
                    list.insert(contact.pk, contact);
                }
                if let Some(timestamp) = contact_list_timestamp {
                    if timestamp < event.created_at {
                        contact_list = list;
                        contact_list_timestamp = Some(event.created_at);
                    }
                } else {
                    contact_list = list;
                    contact_list_timestamp = Some(event.created_at);
                }
            }
            for contact in contact_list.values() {
                let filter = SubscriptionFilter::new()
                    .authors(vec![contact.pk])
                    .kind(Kind::Metadata)
                    .limit(1);
                let timeout = Duration::from_secs(10);
                let events = client.get_events_of(vec![filter], Some(timeout)).await?;
                if let Some(event) = events.first() {
                    let metadata: Metadata = serde_json::from_str(event.content.as_str())?;
                    map.insert(contact.pk, Some(metadata));
                }
            }
            contact::store(&Contacts {
                contacts: map.clone(),
                updated_at: Some(now),
            })?;

            map
        }
    };

    for (pk, metadata) in contact_list {
        match metadata {
            Some(Metadata {
                name: Some(name), ..
            }) => print!("{name} "),
            Some(_) | None => print!("{pk} "),
        }
        println!("{pk}");
    }

    Ok(())
}
