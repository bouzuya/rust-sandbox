use anyhow::Context as _;

use crate::client::{CalendarEventTime, Client};

#[derive(clap::Args)]
pub struct Args {
    event_id: String,
}

#[derive(serde::Deserialize)]
struct Config {
    pub calendar_id: String,
    pub debug: bool,
    // env GOOGLE_APPLICATION_CREDENTIALS
    pub impersonate_user_email: Option<String>,
}

pub async fn execute(Args { event_id }: Args) -> anyhow::Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("net.bouzuya.rust-sandbox.yotei")?;
    let config_file_path = xdg_dirs
        .place_config_file("config.json")
        .context("The parent directory of the config file could not be created")?;
    let config_file_content = std::fs::read_to_string(&config_file_path)
        .with_context(|| format!("The config file could not be read ({:?})", config_file_path))?;
    let config = serde_json::from_str::<Config>(&config_file_content)
        .context("The config file could not be parsed")?;

    let client = Client::new(config.debug, config.impersonate_user_email).await?;
    let get_event_response = client.get_event(&config.calendar_id, &event_id).await?;

    fn time_to_string(event_time: &CalendarEventTime) -> String {
        match &event_time.date {
            Some(d) => d.to_string(),
            None => match &event_time.date_time {
                Some(dt) => dt.to_owned(),
                None => "".to_owned(),
            },
        }
    }

    let item = get_event_response;
    println!(
        "{} {} {}/{}",
        item.id.context("id not found")?,
        item.summary.context("summary not found")?,
        item.start.as_ref().map(time_to_string).unwrap_or_default(),
        item.end.as_ref().map(time_to_string).unwrap_or_default(),
    );

    Ok(())
}
