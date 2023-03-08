use anyhow::Context;
use nostr_sdk::prelude::ToBech32;
use time::format_description::well_known::Rfc3339;

use crate::{client::new_client, event_id::event_id_from_hex_or_bech32, metadata_cache};

pub async fn get(event_id: String) -> anyhow::Result<()> {
    let event_id = event_id_from_hex_or_bech32(event_id.as_str())?;
    let client = new_client().await?;
    let event = client
        .get_text_note(event_id)
        .await?
        .with_context(|| format!("event ({event_id:?}) not found"))?;

    let metadata = {
        let public_key = event.pubkey;
        let mut metadata_cache = metadata_cache::load()?;
        let metadata = metadata_cache.update(public_key, &client).await?;
        metadata_cache::store(&metadata_cache)?;
        metadata
    };

    println!(
        "@{} ({}) : ",
        metadata
            .and_then(|m| m.name)
            .unwrap_or(event.pubkey.to_bech32()?),
        event.pubkey.to_bech32()?
    );
    println!("{}", event.content);
    println!(
        "{} {}",
        time::OffsetDateTime::from_unix_timestamp(event.created_at.as_i64())?.format(&Rfc3339)?,
        event.id.to_bech32()?
    );
    println!();
    Ok(())
}
