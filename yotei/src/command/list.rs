use anyhow::Context as _;

use crate::client::{CalendarEventTime, Client};

#[derive(clap::Args)]
pub struct Args {
    #[arg(env)]
    calendar_id: String,
    #[arg(env, long)]
    debug: bool,
    // env GOOGLE_APPLICATION_CREDENTIALS
    #[arg(env = "EMAIL")]
    impersonate_user_email: Option<String>,
}

pub async fn execute(
    Args {
        calendar_id,
        debug,
        impersonate_user_email,
    }: Args,
) -> anyhow::Result<()> {
    let client = Client::new(debug, impersonate_user_email).await?;
    let list_events_response = client.list_events(&calendar_id).await?;

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
