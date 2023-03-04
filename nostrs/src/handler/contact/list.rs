use std::{collections::HashMap, time::Duration};

use nostr_sdk::prelude::{Kind, Metadata, SubscriptionFilter, Timestamp};

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
            let contact_list = client.get_contact_list().await?;
            for contact in contact_list {
                let filter = SubscriptionFilter::new()
                    .authors(vec![contact.pk])
                    .kind(Kind::Metadata)
                    .limit(1);
                let timeout = Duration::from_secs(10);
                if let Some(event) = client.get_event_of(vec![filter], Some(timeout)).await? {
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
