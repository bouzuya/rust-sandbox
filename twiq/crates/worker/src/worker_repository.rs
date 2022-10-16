use async_trait::async_trait;
use event_store_core::event_id::EventId;

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

#[async_trait]
pub trait WorkerRepository {
    async fn find_last_event_id(&self, worker_name: WorkerName) -> Result<Option<EventId>>;
    async fn store_last_event_id(
        &self,
        worker_name: WorkerName,
        before: Option<EventId>,
        after: EventId,
    ) -> Result<()>;
}

pub trait HasWorkerRepository {
    type WorkerRepository: WorkerRepository + Send + Sync;

    fn worker_repository(&self) -> &Self::WorkerRepository;
}
