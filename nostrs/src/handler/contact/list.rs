use nostr_sdk::prelude::{Metadata, ToBech32};

use crate::{
    client::new_client,
    metadata_cache::{self},
};

pub async fn handle() -> anyhow::Result<()> {
    let client = new_client().await?;

    let mut metadata_cache = metadata_cache::load()?;
    let contact_list = client.get_contact_list().await?;
    for contact in contact_list {
        let public_key = contact.pk;
        let metadata = match metadata_cache.get(public_key) {
            Some(metadata) => Some(metadata),
            None => {
                let metadata = client.get_metadata(public_key).await?;
                if let Some(metadata) = metadata.clone() {
                    metadata_cache.set(public_key, metadata);
                }
                metadata
            }
        };

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
