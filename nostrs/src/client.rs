use std::env;

use nostr_sdk::{
    prelude::{FromSkStr, Keys},
    Client, RelayOptions,
};

use crate::config;

pub async fn new_client() -> anyhow::Result<Client> {
    let my_keys = Keys::from_sk_str(env::var("SECRET_KEY")?.as_str())?;

    let client = Client::new(&my_keys);
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
