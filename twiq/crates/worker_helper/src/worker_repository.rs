use std::fmt::Display;

use async_trait::async_trait;
use event_store_core::{event_id::EventId, Event};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorkerName {
    CreateUserRequest,
    UpdateQueryUser,
    UpdateUser,
    SendUserRequest,
}

impl Display for WorkerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorkerName::CreateUserRequest => "create_user_request",
                WorkerName::UpdateQueryUser => "update_query_user",
                WorkerName::UpdateUser => "update_user",
                WorkerName::SendUserRequest => "send_user_request",
            }
        )
    }
}

#[async_trait]
pub trait WorkerRepository {
    async fn find_last_event_id(&self, worker_name: WorkerName) -> Result<Option<EventId>>;
    async fn store_last_event_id(
        &self,
        worker_name: WorkerName,
        before: Option<EventId>,
        after: EventId,
    ) -> Result<()>;
    //
    async fn find_event_ids(&self, event_id: Option<EventId>) -> Result<Vec<EventId>>;
    async fn find_event(&self, event_id: EventId) -> Result<Option<Event>>;
}

pub trait HasWorkerRepository {
    type WorkerRepository: WorkerRepository + Send + Sync;

    fn worker_repository(&self) -> &Self::WorkerRepository;
}
