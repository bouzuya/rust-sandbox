use std::env;

use domain::aggregate::user_request::value::user_response::UserResponse;
use reqwest::{Client, Method, Url};

use crate::{
    event_store::HasEventStore,
    user_request_repository::{HasUserRequestRepository, UserRequestRepository},
    worker_repository::{HasWorkerRepository, WorkerName},
};

use super::worker_helper;

pub struct Command;

async fn get_user(bearer_token: &str, user_id: &str) -> Result<(u16, String), reqwest::Error> {
    // TODO: error handling
    let mut url = Url::parse("https://api.twitter.com").unwrap();
    url.set_path(&format!("/2/users/{}", user_id));
    let response = Client::builder()
        .build()?
        .request(Method::GET, url)
        .bearer_auth(bearer_token)
        .send()
        .await?;
    let status = response.status().as_u16();
    response.text().await.map(|body| (status, body))
}

async fn handle<C: HasUserRequestRepository>(
    context: &C,
    event: domain::Event,
) -> worker_helper::Result<()> {
    if let domain::Event::UserRequest(domain::aggregate::user_request::Event::Created(event)) =
        event
    {
        let user_request_repository = context.user_request_repository();

        let mut user_request = user_request_repository
            .find(event.user_request_id())
            .await?
            .ok_or_else(|| worker_helper::Error::UserRequestNotFound(event.user_request_id()))?;

        let before = user_request.clone();
        match user_request.start() {
            Ok(_) => {
                user_request_repository
                    .store(Some(before), user_request.clone())
                    .await?;
            }
            Err(_) => {
                return Ok(());
            }
        }

        // TODO: error handling
        let bearer_token = env::var("TWITTER_BEARER_TOKEN").unwrap();
        let user_id = event.user_id().to_string();
        let (status, body) = get_user(&bearer_token, &user_id).await.unwrap();

        user_request.finish(UserResponse::new(status, body))?;
        let before = user_request.clone();
        user_request_repository
            .store(Some(before), user_request)
            .await?;
    }
    Ok(())
}

pub async fn handler<C>(context: &C, _: Command) -> worker_helper::Result<()>
where
    C: HasEventStore + HasUserRequestRepository + HasWorkerRepository,
{
    worker_helper::worker(context, WorkerName::SendUserRequest, handle).await
}

// TODO: test
