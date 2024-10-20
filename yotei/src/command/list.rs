use anyhow::Context as _;

use crate::client::{CalendarEventTime, Client};

pub async fn execute() -> anyhow::Result<()> {
    let debug = false;

    // env GOOGLE_APPLICATION_CREDENTIALS
    let calendar_id = std::env::var("CALENDAR_ID")?;
    let impersonate_user_email = std::env::var_os("EMAIL")
        .map(|s| s.into_string())
        .transpose()
        .map_err(|_| anyhow::anyhow!("EMAIL is not UTF-8"))?;

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
