use std::env;

use async_trait::async_trait;
use domain::aggregate::user_request::value::user_response::UserResponse;
use reqwest::{Client, Method, Url};
use tracing::{info, instrument};
use use_case::user_request_repository::{HasUserRequestRepository, UserRequestRepository};

use crate::worker_repository::WorkerName;

use super::worker_helper::{self, WorkerDeps};

pub struct Command;

pub trait Context: WorkerDeps + HasUserRequestRepository {}

impl<T: WorkerDeps + HasUserRequestRepository> Context for T {}

#[async_trait]
pub trait Has: Context + Sized {
    async fn send_user_request(&self, command: Command) -> worker_helper::Result<()> {
        handler(self, command).await
    }
}

#[instrument(skip_all)]
async fn get_user(bearer_token: &str, user_id: &str) -> Result<(u16, String), reqwest::Error> {
    // TODO: error handling
    let mut url = Url::parse("https://api.twitter.com").unwrap();
    let path = format!("/2/users/{}", user_id);
    url.set_path(&path);
    info!("{} {}", Method::GET, url);
    let response = Client::builder()
        .build()?
        .request(Method::GET, url)
        .bearer_auth(bearer_token)
        .send()
        .await?;
    let status = response.status().as_u16();
    response.text().await.map(|body| (status, body))
}

async fn handle<C: Context>(context: &C, event: domain::Event) -> worker_helper::Result<()> {
    if let domain::Event::UserRequestCreated(event) = event {
        let user_request_repository = context.user_request_repository();

        let user_request = user_request_repository
            .find(event.user_request_id())
            .await?
            .ok_or_else(|| worker_helper::Error::UserRequestNotFound(event.user_request_id()))?;

        let started = match user_request.start() {
            Ok(started) => {
                user_request_repository
                    .store(Some(user_request.clone()), started.clone())
                    .await?;
                started
            }
            Err(_) => {
                return Ok(());
            }
        };

        // TODO: error handling
        let bearer_token = env::var("TWITTER_BEARER_TOKEN").unwrap();
        let twitter_user_id = event.twitter_user_id().to_string();
        let (status, body) = get_user(&bearer_token, &twitter_user_id).await.unwrap();

        let finished = started.finish(UserResponse::new(status, body))?;
        user_request_repository
            .store(Some(started), finished)
            .await?;
    }
    Ok(())
}

pub async fn handler<C: Context>(context: &C, _: Command) -> worker_helper::Result<()> {
    worker_helper::worker(context, WorkerName::SendUserRequest, handle).await
}

// TODO: test