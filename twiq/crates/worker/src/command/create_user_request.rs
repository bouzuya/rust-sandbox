use async_trait::async_trait;
use command_handler::user_request_repository::{HasUserRequestRepository, UserRequestRepository};
use domain::aggregate::user_request::UserRequest;

use ::worker_helper::{
    worker_helper::{self, WorkerDeps},
    worker_repository::WorkerName,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("event {0}")]
    Event(#[from] domain::event::Error),
    #[error("event_store {0}")]
    EventStore(#[from] command_handler::event_store::Error),
    #[error("user_request_aggregate {0}")]
    UserRequestAggregate(#[from] domain::aggregate::user_request::Error),
    #[error("user_request_repository {0}")]
    UserRequestRepository(#[from] command_handler::user_request_repository::Error),
    #[error("worker_repository {0}")]
    WorkerRepository(#[from] ::worker_helper::worker_repository::Error),
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Command;

pub trait Context: WorkerDeps + HasUserRequestRepository {}

impl<T: WorkerDeps + HasUserRequestRepository> Context for T {}

#[async_trait]
pub trait Has: Context + Sized {
    async fn create_user_request(&self, command: Command) -> worker_helper::Result<()> {
        handler(self, command).await
    }
}

async fn handle<C: Context>(context: &C, event: domain::Event) -> worker_helper::Result<()> {
    if let domain::Event::UserRequested(event) = event {
        let user_request_repository = context.user_request_repository();
        if user_request_repository
            .find(event.user_request_id())
            .await?
            .is_none()
        {
            let user_request = UserRequest::create(
                event.user_request_id(),
                event.twitter_user_id().clone(),
                event.user_id(),
            )?;
            user_request_repository.store(None, user_request).await?;
        }
    }
    Ok(())
}

pub async fn handler<C: Context>(context: &C, _: Command) -> worker_helper::Result<()> {
    worker_helper::worker(context, WorkerName::CreateUserRequest, handle).await
}

// TODO: test
