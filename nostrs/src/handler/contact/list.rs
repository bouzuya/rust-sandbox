use nostr_sdk::prelude::{Metadata, ToBech32};

use crate::{client::Client, metadata_cache};

pub async fn list() -> anyhow::Result<()> {
    let client = Client::new().await?;

    let mut metadata_cache = metadata_cache::load()?;
    let contact_list = client.get_contact_list().await?;
    for contact in contact_list {
        let public_key = contact.pk;
        let metadata = metadata_cache.update(public_key, &client).await?;

        match metadata {
            Some(Metadata {
                name: Some(name), ..
            }) => print!("{name} "),
            Some(_) | None => print!("(none) "),
        }
        println!("{}", public_key.to_bech32()?);
    }
    metadata_cache::store(&metadata_cache)?;

    Ok(())
}
