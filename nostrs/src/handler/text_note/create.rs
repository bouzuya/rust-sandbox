use nostr_sdk::prelude::{Tag, ToBech32};

use crate::{client::new_client, event_id::event_id_from_hex_or_bech32};

pub async fn handle(content: String, reply_to: Option<String>) -> anyhow::Result<()> {
    let client = new_client().await?;
    let mut options = vec![];
    if let Some(event_id) = reply_to {
        let event_id = event_id_from_hex_or_bech32(event_id.as_str())?;
        options.push(Tag::Event(event_id, None, None));
    }
    let note_id = client
        .publish_text_note(content, &options)
        .await?
        .to_bech32()?;
    println!("{note_id}");
    Ok(())
}
