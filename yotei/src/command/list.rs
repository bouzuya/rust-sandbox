use anyhow::Context as _;

pub async fn execute() -> anyhow::Result<()> {
    // env GOOGLE_APPLICATION_CREDENTIALS
    let default_token_source_provider = google_cloud_auth::token::DefaultTokenSourceProvider::new(
        google_cloud_auth::project::Config::default().with_scopes(&[
            "https://www.googleapis.com/auth/calendar",
            "https://www.googleapis.com/auth/calendar.events",
        ]),
    )
    .await?;
    let project_id = default_token_source_provider
        .project_id
        .clone()
        .context("project_id not found")?;
    let token_source =
        google_cloud_token::TokenSourceProvider::token_source(&default_token_source_provider);

    let token = token_source.token().await.map_err(|e| anyhow::anyhow!(e))?;

    // println!("project_id : {:?}", project_id);
    // println!("token      : {:?}", token);

    let calendar_id = std::env::var("CALENDAR_ID")?;
    let response = reqwest::Client::new()
        .get(format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events",
            calendar_id
        ))
        .header(reqwest::header::AUTHORIZATION, token.clone())
        .send()
        .await?;
    println!("{}", response.status());
    println!("{}", response.text().await?);

    Ok(())
}
