use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventStoreError {
    #[error("InsertAggregate")]
    InsertAggregate,
    #[error("InsertEvent")]
    InsertEvent,
    #[error("InvalidAggregateId")]
    InvalidAggregateId,
    #[error("InvalidAggregateVersion")]
    InvalidAggregateVersion,
    #[error("IO")]
    IO,
    #[error("MigrateCreateAggregateTable")]
    MigrateCreateAggregateTable,
    #[error("MigrateCreateEventTable")]
    MigrateCreateEventTable,
    #[error("UpdateAggregate")]
    UpdateAggregate,
    #[error("Unknown")]
    Unknown,
}
