use std::env;

use nostr_sdk::{
    prelude::{FromSkStr, Keys},
    Client,
};

pub async fn new_client() -> anyhow::Result<Client> {
    let my_keys = Keys::from_sk_str(env::var("SECRET_KEY")?.as_str())?;

    let client = Client::new(&my_keys);

    client
        .add_relay("wss://nostr-pub.wellorder.net", None)
        .await?;
    client.connect().await;

    Ok(client)
}
