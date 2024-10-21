use anyhow::Context as _;

use crate::{
    client::{CalendarEventTime, Client},
    config::Config,
};

#[derive(clap::Args)]
pub struct Args {
    #[arg(long)]
    end_date_time: String,
    #[arg(long)]
    start_date_time: String,
    #[arg(long)]
    summary: String,
}

pub async fn execute(
    Args {
        end_date_time,
        start_date_time,
        summary,
    }: Args,
) -> anyhow::Result<()> {
    let config = Config::load()?;
    let client = Client::new(config.debug, config.impersonate_user_email).await?;
    let insert_event_response = client
        .insert_event(
            &config.calendar_id,
            &summary,
            &start_date_time,
            &end_date_time,
        )
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
