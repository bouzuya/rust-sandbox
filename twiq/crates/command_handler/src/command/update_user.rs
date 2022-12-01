use async_trait::async_trait;

use crate::{
    user_repository::{HasUserRepository, UserRepository},
    user_request_repository::{HasUserRequestRepository, UserRequestRepository},
};

use ::worker_helper::{
    worker_helper::{self, WorkerDeps},
    worker_repository::WorkerName,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("user_aggregate {0}")]
    UserAggregate(#[from] domain::aggregate::user::Error),
    #[error("user_repository {0}")]
    UserRepository(#[from] crate::user_repository::Error),
    #[error("user_request_repository {0}")]
    UserRequestRepository(#[from] crate::user_request_repository::Error),
    #[error("user_response {0}")]
    UserResponse(#[from] domain::aggregate::user_request::value::user_response::Error),
    #[error("worker_helper {0}")]
    WorkerHelper(#[from] worker_helper::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Command;

pub trait Context: WorkerDeps + HasUserRepository + HasUserRequestRepository {}

impl<T: WorkerDeps + HasUserRepository + HasUserRequestRepository> Context for T {}

#[async_trait]
pub trait Has: Context + Sized {
    async fn update_user(&self, command: Command) -> Result<()> {
        handler(self, command).await
    }
}

async fn handle<C: Context>(
    context: &C,
    event: domain::Event,
) -> Result<(), Box<dyn std::error::Error>> {
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

pub async fn handler<C: Context>(context: &C, _: Command) -> Result<()> {
    Ok(worker_helper::worker(context, WorkerName::UpdateUser, handle).await?)
}

// TODO: test
