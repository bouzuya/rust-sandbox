use std::sync::Arc;

use anyhow::Context as _;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventTime {
    pub date: Option<String>,
    pub date_time: Option<String>,
    pub time_zone: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    pub end: Option<CalendarEventTime>,
    pub id: Option<String>,
    pub start: Option<CalendarEventTime>,
    pub summary: Option<String>,
}

pub type GetEventResponse = CalendarEvent;

pub type InsertEventResponse = CalendarEvent;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEventsResponse {
    pub items: Vec<CalendarEvent>,
}

pub struct Client {
    client: reqwest::Client,
    debug: bool,
    token_source: Arc<dyn google_cloud_token::TokenSource>,
}

impl Client {
    pub async fn new(debug: bool, impersonate_user_email: Option<String>) -> anyhow::Result<Self> {
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

        if debug {
            println!("project_id : {:?}", project_id);
        }

        let client = reqwest::Client::new();
        Ok(Self {
            client,
            debug,
            token_source,
        })
    }

    pub async fn delete_event(&self, calendar_id: &str, event_id: &str) -> anyhow::Result<()> {
        let _ = self
            .request(
                reqwest::Method::DELETE,
                format!(
                    "https://www.googleapis.com/calendar/v3/calendars/{}/events/{}",
                    calendar_id, event_id
                ),
                None::<&()>,
            )
            .await?;
        Ok(())
    }

    pub async fn get_event(
        &self,
        calendar_id: &str,
        event_id: &str,
    ) -> anyhow::Result<GetEventResponse> {
        let response = self
            .request(
                reqwest::Method::GET,
                format!(
                    "https://www.googleapis.com/calendar/v3/calendars/{}/events/{}",
                    calendar_id, event_id
                ),
                None::<&()>,
            )
            .await?;
        let response_body = response.text().await?;
        if self.debug {
            println!("{}", response_body);
        }

        Ok(serde_json::from_str(&response_body)?)
    }

    pub async fn insert_event(
        &self,
        calendar_id: &str,
        summary: &str,
        start_date_time: &str,
        end_date_time: &str,
    ) -> anyhow::Result<InsertEventResponse> {
        let response = self
            .request(
                reqwest::Method::POST,
                format!(
                    "https://www.googleapis.com/calendar/v3/calendars/{}/events?conferenceDataVersion=1",
                    calendar_id
                ) ,
                Some(&serde_json::json!({
                    // required properties
                    "end": {
                        "dateTime": end_date_time
                    },
                    "start": {
                        "dateTime": start_date_time
                    },

                    // optional properties
                    // "attendees": [
                    //     {
                    //         "email": "m@bouzuya.net"
                    //     }
                    // ]
                    "summary": summary
                }))
            )
            .await?;
        let response_body = response.text().await?;
        if self.debug {
            println!("{}", response_body);
        }

        Ok(serde_json::from_str(&response_body)?)
    }

    pub async fn list_events(&self, calendar_id: &str) -> anyhow::Result<ListEventsResponse> {
        let response = self
            .request(
                reqwest::Method::GET,
                format!(
                    "https://www.googleapis.com/calendar/v3/calendars/{}/events",
                    calendar_id
                ),
                None::<&()>,
            )
            .await?;
        let response_body = response.text().await?;
        if self.debug {
            println!("{}", response_body);
        }

        Ok(serde_json::from_str(&response_body)?)
    }

    async fn request<T>(
        &self,
        method: reqwest::Method,
        url: String,
        body: Option<&T>,
    ) -> anyhow::Result<reqwest::Response>
    where
        T: serde::Serialize,
    {
        let token = self.token().await?;
        let request = self
            .client
            .request(method, url)
            .header(reqwest::header::AUTHORIZATION, token);
        let response = match body {
            Some(body) => request.json(body),
            None => request,
        }
        .send()
        .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "status code is not success ({} {})",
                response.status(),
                response.text().await?
            );
        }

        Ok(response)
    }

    async fn token(&self) -> anyhow::Result<String> {
        self.token_source
            .token()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}
