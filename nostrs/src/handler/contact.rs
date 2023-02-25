use std::{collections::HashMap, time::Duration};

use anyhow::bail;
use nostr_sdk::prelude::{Contact, Event, Kind, SubscriptionFilter, Tag, Timestamp};

use crate::client::new_client;

pub async fn list() -> anyhow::Result<()> {
    let client = new_client().await?;

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
        println!("{}", serde_json::to_string(contact)?);
    }

    Ok(())
}
