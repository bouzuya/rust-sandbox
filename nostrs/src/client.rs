use std::{collections::HashMap, time::Duration};

use nostr_sdk::{
    prelude::{Event, FromSkStr, Keys, SubscriptionFilter},
    Client, Options, RelayOptions,
};

use crate::{config, keypair};

pub async fn new_client() -> anyhow::Result<Client> {
    let private_key = keypair::load()?;
    let my_keys = Keys::from_sk_str(private_key.as_str())?;

    let client = Client::new_with_opts(&my_keys, Options::default().wait_for_send(true));
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

    Ok(client)
}

/// Returns only one event with the latest created_at.
pub async fn get_event_of(
    client: &Client,
    filters: Vec<SubscriptionFilter>,
    timeout: Option<Duration>,
) -> anyhow::Result<Option<Event>> {
    // the events is in ascending order by created_at.
    let events = get_events_of(client, filters, timeout).await?;
    Ok(events.last().cloned())
}

/// Returns events in ascending order by created_at, with duplicate id's removed.
pub async fn get_events_of(
    client: &Client,
    filters: Vec<SubscriptionFilter>,
    timeout: Option<Duration>,
) -> anyhow::Result<Vec<Event>> {
    let events = client.get_events_of(filters, timeout).await?;
    let mut map = HashMap::new();
    for event in events {
        map.insert(event.id, event);
    }
    let mut events = map.into_values().collect::<Vec<Event>>();
    events.sort_by_key(|event| event.created_at);
    Ok(events)
}

#[cfg(test)]
mod tests {
    use nostr_sdk::prelude::Kind;

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let client = new_client().await?;
        let filter = SubscriptionFilter::new()
            .authors(vec![client.keys().public_key()])
            .kind(Kind::ContactList);
        let timeout = Duration::from_secs(10);
        let events = get_events_of(&client, vec![filter], Some(timeout)).await?;
        println!("{}", serde_json::to_string_pretty(&events)?);
        Ok(())
    }
}
