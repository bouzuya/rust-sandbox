use crate::{client::Client, config::Config};

#[derive(clap::Args)]
pub struct Args {
    event_id: String,
}

pub async fn execute(Args { event_id }: Args) -> anyhow::Result<()> {
    let config = Config::load()?;
    let client = Client::new(config.debug, config.impersonate_user_email).await?;
    let _ = client.delete_event(&config.calendar_id, &event_id).await?;
    println!("{} deleted", event_id);
    Ok(())
}
