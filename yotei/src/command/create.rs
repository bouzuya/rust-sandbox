use anyhow::Context as _;

use crate::client::{CalendarEventTime, Client};

#[derive(clap::Args)]
pub struct Args {
    #[arg(env)]
    calendar_id: String,
    #[arg(env, long)]
    debug: bool,
    #[arg(long)]
    end_date_time: String,
    // env GOOGLE_APPLICATION_CREDENTIALS
    #[arg(env = "EMAIL")]
    impersonate_user_email: Option<String>,
    #[arg(long)]
    start_date_time: String,
    #[arg(long)]
    summary: String,
}

pub async fn execute(
    Args {
        calendar_id,
        debug,
        end_date_time,
        impersonate_user_email,
        start_date_time,
        summary,
    }: Args,
) -> anyhow::Result<()> {
    let client = Client::new(debug, impersonate_user_email).await?;
    let insert_event_response = client
        .insert_event(&calendar_id, &summary, &start_date_time, &end_date_time)
        .await?;

    fn time_to_string(event_time: &CalendarEventTime) -> String {
        match &event_time.date {
            Some(d) => d.to_string(),
            None => match &event_time.date_time {
                Some(dt) => dt.to_owned(),
                None => "".to_owned(),
            },
        }
    }

    let item = insert_event_response;
    println!(
        "{} {} {}/{}",
        item.id.context("id not found")?,
        item.summary.context("summary not found")?,
        item.start.as_ref().map(time_to_string).unwrap_or_default(),
        item.end.as_ref().map(time_to_string).unwrap_or_default(),
    );

    Ok(())
}
