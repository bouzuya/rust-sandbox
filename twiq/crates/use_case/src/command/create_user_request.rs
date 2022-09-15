use domain::aggregate::user_request::UserRequest;

use crate::{
    event_store::{EventStore, HasEventStore},
    user_request_repository::{HasUserRequestRepository, UserRequestRepository},
    worker_repository::{HasWorkerRepository, WorkerName, WorkerRepository},
};

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

pub async fn handler<C: HasEventStore + HasUserRequestRepository + HasWorkerRepository>(
    context: &C,
    _: Command,
) -> Result<()> {
    let event_store = context.event_store();
    let worker_name = WorkerName::CreateUserRequest;
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
        if let domain::Event::User(domain::aggregate::user::Event::Requested(event)) = event {
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
        worker_repository
            .store_last_event_id(worker_name, last_event_id, event_id)
            .await?;
        last_event_id = Some(event_id);
    }
    Ok(())
}

// TODO: test
