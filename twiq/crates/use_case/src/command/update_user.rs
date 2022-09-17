use std::future::Future;

use domain::aggregate::user::{UserId, UserRequestId};

use crate::{
    event_store::{EventStore, HasEventStore},
    user_repository::{HasUserRepository, UserRepository},
    user_request_repository::{HasUserRequestRepository, UserRequestRepository},
    worker_repository::{HasWorkerRepository, WorkerName, WorkerRepository},
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("event {0}")]
    Event(#[from] domain::event::Error),
    #[error("event_store {0}")]
    EventStore(#[from] crate::event_store::Error),
    #[error("user_aggregate {0}")]
    UserAggregate(#[from] domain::aggregate::user::Error),
    #[error("user not found {0}")]
    UserNotFound(UserId),
    #[error("user_repository {0}")]
    UserRepository(#[from] crate::user_repository::Error),
    #[error("user_request_aggregate {0}")]
    UserRequestAggregate(#[from] domain::aggregate::user_request::Error),
    #[error("user_request not found {0}")]
    UserRequestNotFound(UserRequestId),
    #[error("user_request_repository {0}")]
    UserRequestRepository(#[from] crate::user_request_repository::Error),
    #[error("user_response {0}")]
    UserResponse(#[from] domain::aggregate::user_request::value::user_response::Error),
    #[error("worker_repository {0}")]
    WorkerRepository(#[from] crate::worker_repository::Error),
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Command;

async fn handle<C: HasUserRepository + HasUserRequestRepository>(
    context: &C,
    event: domain::Event,
) -> Result<()> {
    if let domain::Event::UserRequest(domain::aggregate::user_request::Event::Finished(event)) =
        event
    {
        let user_repository = context.user_repository();
        let user_request_repository = context.user_request_repository();
        let user_request = user_request_repository
            .find(event.user_request_id())
            .await?;
        let user = user_repository.find(event.user_id()).await?;
        match (user_request, user) {
            (None, _) => Err(Error::UserRequestNotFound(event.user_request_id())),
            (_, None) => Err(Error::UserNotFound(event.user_id())),
            (Some(_), Some(user)) => {
                let twitter_user_name = event.user_response().parse()?;
                let at = event.at();
                let mut updated = user.clone();
                updated.update(twitter_user_name, at)?;
                Ok(user_repository.store(Some(user), updated).await?)
            }
        }?;
    }
    Ok(())
}

async fn worker<'a, C, F, Fut>(context: &'a C, worker_name: WorkerName, handle: F) -> Result<()>
where
    C: HasEventStore + HasWorkerRepository,
    F: Fn(&'a C, domain::Event) -> Fut,
    Fut: Future<Output = Result<()>>,
{
    let event_store = context.event_store();
    let worker_repository = context.worker_repository();
    let mut last_event_id = worker_repository.find_last_event_id(worker_name).await?;
    let event_ids = match last_event_id {
        None => event_store.find_event_ids().await?,
        Some(event_id) => {
            event_store
                .find_event_ids_by_event_id_after(event_id)
                .await?
        }
    };
    for event_id in event_ids {
        let event = event_store
            .find_event(event_id)
            .await?
            .ok_or_else(|| Error::Unknown("event not found".to_owned()))?;
        let event = domain::Event::try_from(event)?;

        handle(context, event).await?;

        worker_repository
            .store_last_event_id(worker_name, last_event_id, event_id)
            .await?;
        last_event_id = Some(event_id);
    }
    Ok(())
}

pub async fn handler<C>(context: &C, _: Command) -> Result<()>
where
    C: HasEventStore + HasUserRepository + HasUserRequestRepository + HasWorkerRepository,
{
    worker(context, WorkerName::CreateUserRequest, handle).await
}

// TODO: test
