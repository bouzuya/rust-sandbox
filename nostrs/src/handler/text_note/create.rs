use crate::client::new_client;

pub async fn handle(content: String) -> anyhow::Result<()> {
    let client = new_client().await?;
    let event_id = client.publish_text_note(content, &[]).await?;
    println!("{event_id:?}");
    Ok(())
}
