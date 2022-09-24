use domain::aggregate::user_request::UserRequest;

use crate::{
    user_request_repository::{HasUserRequestRepository, UserRequestRepository},
    worker_repository::WorkerName,
};

use super::worker_helper::{self, WorkerDeps};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("event {0}")]
    Event(#[from] domain::event::Error),
    #[error("event_store {0}")]
    EventStore(#[from] crate::event_store::Error),
    #[error("user_request_aggregate {0}")]
    UserRequestAggregate(#[from] domain::aggregate::user_request::Error),
    #[error("user_request_repository {0}")]
    UserRequestRepository(#[from] crate::user_request_repository::Error),
    #[error("worker_repository {0}")]
    WorkerRepository(#[from] crate::worker_repository::Error),
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Command;

async fn handle<C: HasUserRequestRepository>(
    context: &C,
    event: domain::Event,
) -> worker_helper::Result<()> {
    if let domain::Event::UserRequested(event) = event {
        let user_request_repository = context.user_request_repository();
        if user_request_repository
            .find(event.user_request_id())
            .await?
            .is_none()
        {
            let user_request = UserRequest::create(
                event.user_request_id(),
                event.twitter_user_id(),
                event.user_id(),
            )?;
            user_request_repository.store(None, user_request).await?;
        }
    }
    Ok(())
}

pub async fn handler<C: WorkerDeps + HasUserRequestRepository>(
    context: &C,
    _: Command,
) -> worker_helper::Result<()> {
    worker_helper::worker(context, WorkerName::CreateUserRequest, handle).await
}

// TODO: test
