use std::{collections::HashMap, time::Duration};

use anyhow::{bail, Context};
use nostr_sdk::{
    prelude::{
        Contact, Event, EventId, FromSkStr, Keys, Kind, Metadata, SubscriptionFilter, Tag,
        Timestamp, XOnlyPublicKey,
    },
    Options, RelayOptions,
};

use crate::{config, keypair};

pub struct Client {
    client: nostr_sdk::Client,
    timeout: Duration,
}

impl Client {
    pub async fn new() -> anyhow::Result<Self> {
        let private_key = keypair::load()?;
        let my_keys = Keys::from_sk_str(private_key.as_str())?;

        let client =
            nostr_sdk::Client::new_with_opts(&my_keys, Options::default().wait_for_send(true));
        let config = config::load()?;
        for (url, options) in config.relays.iter() {
            client
                .add_relay_with_opts(
                    url.as_str(),
                    None,
                    RelayOptions::new(options.read, options.write),
                )
                .await?;
        }
        client.connect().await;

        let timeout = Duration::from_secs(10);
        Ok(Self { client, timeout })
    }

    pub async fn delete_event(&self, event_id: EventId) -> anyhow::Result<EventId> {
        Ok(self.client.delete_event::<String>(event_id, None).await?)
    }

    pub async fn dislike(&self, event_id: EventId) -> anyhow::Result<EventId> {
        let public_key = self.get_text_note_public_key_by_event_id(event_id).await?;
        Ok(self.client.dislike(event_id, public_key).await?)
    }

    pub async fn get_contact_list(&self) -> anyhow::Result<Vec<Contact>> {
        let filter = SubscriptionFilter::new()
            .authors(vec![self.client.keys().public_key()])
            .kind(Kind::ContactList)
            .limit(1);
        let event = self
            .get_event_of(vec![filter], Some(self.timeout))
            .await?
            .context("contact_list not found")?;
        let mut map = HashMap::new();
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
            map.insert(contact.pk, contact);
        }
        Ok(map.into_values().collect::<Vec<Contact>>())
    }

    /// Returns events in ascending order by created_at, with duplicate id's removed.
    pub async fn get_events_of(
        &self,
        filters: Vec<SubscriptionFilter>,
        timeout: Option<Duration>,
    ) -> anyhow::Result<Vec<Event>> {
        let events = self.client.get_events_of(filters, timeout).await?;
        let mut map = HashMap::new();
        for event in events {
            map.insert(event.id, event);
        }
        let mut events = map.into_values().collect::<Vec<Event>>();
        events.sort_by_key(|event| event.created_at);
        Ok(events)
    }

    pub async fn get_metadata(
        &self,
        public_key: XOnlyPublicKey,
        since: Option<Timestamp>,
    ) -> anyhow::Result<Option<Metadata>> {
        let filter = SubscriptionFilter::new()
            .author(public_key)
            .kind(Kind::Metadata)
            .limit(1);
        let filter = match since {
            Some(since) => filter.since(since),
            None => filter,
        };
        let event = self.get_event_of(vec![filter], Some(self.timeout)).await?;
        Ok(if let Some(event) = event {
            let metadata: Metadata = serde_json::from_str(event.content.as_str())?;
            Some(metadata)
        } else {
            None
        })
    }

    pub async fn get_text_note(&self, id: EventId) -> anyhow::Result<Option<Event>> {
        let filter = SubscriptionFilter::new()
            .kind(Kind::TextNote)
            .id(id.to_hex())
            .limit(1);
        self.get_event_of(vec![filter], Some(self.timeout)).await
    }

    pub fn keys(&self) -> Keys {
        self.client.keys()
    }

    pub async fn like(&self, event_id: EventId) -> anyhow::Result<EventId> {
        let public_key = self.get_text_note_public_key_by_event_id(event_id).await?;
        Ok(self.client.like(event_id, public_key).await?)
    }

    pub async fn publish_text_note(
        &self,
        content: String,
        tags: &[Tag],
    ) -> anyhow::Result<EventId> {
        Ok(self.client.publish_text_note(content, tags).await?)
    }

    /// Returns only one event with the latest created_at.
    async fn get_event_of(
        &self,
        filters: Vec<SubscriptionFilter>,
        timeout: Option<Duration>,
    ) -> anyhow::Result<Option<Event>> {
        // the events is in ascending order by created_at.
        let events = self.get_events_of(filters, timeout).await?;
        Ok(events.last().cloned())
    }

    async fn get_text_note_public_key_by_event_id(
        &self,
        event_id: EventId,
    ) -> anyhow::Result<XOnlyPublicKey> {
        let event = self
            .get_text_note(event_id)
            .await?
            .with_context(|| format!("event ({event_id:?}) not found"))?;
        Ok(event.pubkey)
    }
}

#[cfg(test)]
mod tests {
    use nostr_sdk::prelude::Kind;

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let client = Client::new().await?;
        let filter = SubscriptionFilter::new()
            .authors(vec![client.keys().public_key()])
            .kind(Kind::ContactList);
        let timeout = Duration::from_secs(10);
        let events = client.get_events_of(vec![filter], Some(timeout)).await?;
        println!("{}", serde_json::to_string_pretty(&events)?);
        Ok(())
    }
}
