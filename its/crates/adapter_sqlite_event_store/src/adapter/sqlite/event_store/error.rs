#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("InsertEventStream")]
    InsertEventStream,
    #[error("InsertEvent")]
    InsertEvent,
    #[error("InvalidEventId")]
    InvalidEventId,
    #[error("InvalidEventSeq")]
    InvalidEventSeq,
    #[error("InvalidEventStreamId")]
    InvalidEventStreamId,
    #[error("InvalidEventStreamVersion")]
    InvalidEventStreamVersion,
    #[error("IO")]
    IO,
    #[error("SqlxError")]
    SqlxError(#[from] sqlx::Error),
    #[error("UpdateEventStream")]
    UpdateEventStream,
    #[error("Unknown")]
    Unknown,
}
