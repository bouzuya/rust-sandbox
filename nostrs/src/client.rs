use nostr_sdk::{
    prelude::{FromSkStr, Keys},
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
