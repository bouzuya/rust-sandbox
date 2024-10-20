use anyhow::Context as _;

pub async fn execute() -> anyhow::Result<()> {
    let debug = false;

    // env GOOGLE_APPLICATION_CREDENTIALS
    let calendar_id = std::env::var("CALENDAR_ID")?;
    let impersonate_user_email = std::env::var_os("EMAIL")
        .map(|s| s.into_string())
        .transpose()
        .map_err(|_| anyhow::anyhow!("EMAIL is not UTF-8"))?;

    let config = google_cloud_auth::project::Config::default().with_scopes(&[
        "https://www.googleapis.com/auth/calendar",
        "https://www.googleapis.com/auth/calendar.events",
    ]);
    let config = match impersonate_user_email.as_ref() {
        Some(sub) => config.with_sub(sub),
        None => config,
    };
    let default_token_source_provider =
        google_cloud_auth::token::DefaultTokenSourceProvider::new(config).await?;
    let project_id = default_token_source_provider
        .project_id
        .clone()
        .context("project_id not found")?;
    let token_source =
        google_cloud_token::TokenSourceProvider::token_source(&default_token_source_provider);

    let token = token_source.token().await.map_err(|e| anyhow::anyhow!(e))?;

    if debug {
        println!("project_id : {:?}", project_id);
        println!("token      : {:?}", token);
    }

    let response = reqwest::Client::new()
        .get(format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events",
            calendar_id
        ))
        .header(reqwest::header::AUTHORIZATION, token.clone())
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("status code is not success");
    }

    let response_body = response.text().await?;
    if debug {
        println!("{}", response_body);
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CalendarEventTime {
        date: Option<String>,
        date_time: Option<String>,
        // time_zone: Option<String>,
    }
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct CalendarEvent {
        end: Option<CalendarEventTime>,
        id: Option<String>,
        start: Option<CalendarEventTime>,
        summary: Option<String>,
    }
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ListResponse {
        items: Vec<CalendarEvent>,
    }

    fn time_to_string(event_time: &CalendarEventTime) -> String {
        match &event_time.date {
            Some(d) => d.to_string(),
            None => match &event_time.date_time {
                Some(dt) => dt.to_owned(),
                None => "".to_owned(),
            },
        }
    }

    let response_body: ListResponse = serde_json::from_str(&response_body)?;
    for item in response_body.items {
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
