use anyhow::Context as _;

use crate::{
    client::{CalendarEventTime, Client},
    config::Config,
};

#[derive(clap::Args)]
pub struct Args;

pub async fn execute(_: Args) -> anyhow::Result<()> {
    let config = Config::load()?;
    let client = Client::new(config.debug, config.impersonate_user_email).await?;
    let list_events_response = client.list_events(&config.calendar_id).await?;

    fn time_to_string(event_time: &CalendarEventTime) -> String {
        match &event_time.date {
            Some(d) => d.to_string(),
            None => match &event_time.date_time {
                Some(dt) => dt.to_owned(),
                None => "".to_owned(),
            },
        }
    }

    for item in list_events_response.items {
        println!(
            "{} {} {}/{}",
            item.id.context("id not found")?,
            item.summary.context("summary not found")?,
            item.start.as_ref().map(time_to_string).unwrap_or_default(),
            item.end.as_ref().map(time_to_string).unwrap_or_default(),
        );
    }

    Ok(())
}
