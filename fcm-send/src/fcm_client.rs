mod message;
pub mod send;

use anyhow::Context as _;

pub use self::message::*;

use std::sync::Arc;

pub struct FcmClient {
    client: reqwest::Client,
    project_id: String,
    token_source: Arc<dyn google_cloud_token::TokenSource>,
}

impl FcmClient {
    pub async fn new() -> anyhow::Result<Self> {
        let default_token_source_provider =
            google_cloud_auth::token::DefaultTokenSourceProvider::new(
                google_cloud_auth::project::Config::default().with_scopes(&[
                    "https://www.googleapis.com/auth/cloud-platform",
                    "https://www.googleapis.com/auth/firebase.messaging",
                ]),
            )
            .await?;
        let token_source =
            google_cloud_token::TokenSourceProvider::token_source(&default_token_source_provider);
        let project_id = default_token_source_provider
            .project_id
            .context("project_id not found")?;
        let client = reqwest::Client::new();
        Ok(Self {
            client,
            project_id,
            token_source,
        })
    }

    pub fn project_id(&self) -> &str {
        &self.project_id
    }

    /// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages/send>
    pub async fn send(
        &self,
        path_parameters: send::PathParameters,
        request_body: send::RequestBody,
    ) -> Result<send::ResponseBody, Error> {
        self::send::handle(self, path_parameters, request_body).await
    }
}
