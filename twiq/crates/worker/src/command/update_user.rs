use async_trait::async_trait;

use use_case::user_repository::{HasUserRepository, UserRepository};
use use_case::user_request_repository::{HasUserRequestRepository, UserRequestRepository};

use crate::worker_repository::WorkerName;

use super::worker_helper::{self, WorkerDeps};

pub struct Command;

pub trait Context: WorkerDeps + HasUserRepository + HasUserRequestRepository {}

impl<T: WorkerDeps + HasUserRepository + HasUserRequestRepository> Context for T {}

#[async_trait]
pub trait Has: Context + Sized {
    async fn update_user(&self, command: Command) -> worker_helper::Result<()> {
        handler(self, command).await
    }
}

async fn handle<C: Context>(context: &C, event: domain::Event) -> worker_helper::Result<()> {
    if let domain::Event::UserRequestFinished(event) = event {
        let user_repository = context.user_repository();
        let user_request_repository = context.user_request_repository();
        let user_request = user_request_repository
            .find(event.user_request_id())
            .await?;
        let user = user_repository.find(event.user_id()).await?;
        match (user_request, user) {
            (None, _) => Err(worker_helper::Error::UserRequestNotFound(
                event.user_request_id(),
            )),
            (_, None) => Err(worker_helper::Error::UserNotFound(event.user_id())),
            (Some(_), Some(user)) => {
                let twitter_user_name = event.user_response().parse()?;
                let updated = user.update(twitter_user_name)?;
                Ok(user_repository.store(Some(user), updated).await?)
            }
        }?;
    }
    Ok(())
}

pub async fn handler<C: Context>(context: &C, _: Command) -> worker_helper::Result<()> {
    worker_helper::worker(context, WorkerName::UpdateUser, handle).await
}

// TODO: test
