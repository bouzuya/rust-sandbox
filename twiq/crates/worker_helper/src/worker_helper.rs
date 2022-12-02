use std::future::Future;

use crate::worker_repository::{HasWorkerRepository, WorkerName, WorkerRepository};
use domain::aggregate::user::{UserId, UserRequestId};
use tracing::{info, instrument};

pub trait WorkerDeps: HasWorkerRepository {}

impl<T: HasWorkerRepository> WorkerDeps for T {}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("event {0}")]
    Event(#[from] domain::event::Error),
    #[error("user_aggregate {0}")]
    UserAggregate(#[from] domain::aggregate::user::Error),
    #[error("user not found {0}")]
    UserNotFound(UserId),
    #[error("user_request_aggregate {0}")]
    UserRequestAggregate(#[from] domain::aggregate::user_request::Error),
    #[error("user_request not found {0}")]
    UserRequestNotFound(UserRequestId),
    #[error("user_response {0}")]
    UserResponse(#[from] domain::aggregate::user_request::value::user_response::Error),
    #[error("worker_repository {0}")]
    WorkerRepository(#[from] crate::worker_repository::Error),
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[instrument(skip(context, handle))]
pub async fn worker<'a, C, F, Fut>(context: &'a C, worker_name: WorkerName, handle: F) -> Result<()>
where
    C: WorkerDeps,
    F: Fn(&'a C, domain::Event) -> Fut,
    Fut: Future<Output = Result<(), Box<dyn std::error::Error + 'static>>>,
{
    let worker_repository = context.worker_repository();
    let mut last_event_id = worker_repository.find_last_event_id(worker_name).await?;
    info!("last_event_id = {:?}", last_event_id);
    let event_ids = worker_repository.find_event_ids(last_event_id).await?;
    info!("event_ids = {:?}", event_ids);
    for event_id in event_ids {
        if Some(event_id) == last_event_id {
            continue;
        }

        let event = worker_repository
            .find_event(event_id)
            .await?
            .ok_or_else(|| Error::Unknown("event not found".to_owned()))?;
        let event = domain::Event::try_from(event)?;

        handle(context, event)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        worker_repository
            .store_last_event_id(worker_name, last_event_id, event_id)
            .await?;
        last_event_id = Some(event_id);
    }
    Ok(())
}

// TODO: test skip last_event_id
