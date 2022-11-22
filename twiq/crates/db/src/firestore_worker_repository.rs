use std::{collections::HashMap, str::FromStr};

use async_trait::async_trait;
use event_store_core::EventId;
use worker::worker_repository::{self, WorkerName, WorkerRepository};

use crate::{
    config::Config,
    firestore_rpc::{
        google::firestore::v1::{
            precondition::ConditionType, write::Operation, Document, Precondition, Write,
        },
        helper::{get_field_as_str, value_from_string},
    },
    firestore_transaction::FirestoreTransaction,
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("event_store_core::event_id {0}")]
    EventId(#[from] event_store_core::event_id::Error),
    #[error("firestore_rpc::helper {0}")]
    FirestoreRpcHelper(#[from] crate::firestore_rpc::helper::Error),
    #[error("firestore_rpc::helper::GetFieldError {0}")]
    FirestoreRpcHelperGetField(#[from] crate::firestore_rpc::helper::GetFieldError),
    #[error("worker not found {0}")]
    WorkerNotFound(WorkerName),
}

impl From<Error> for worker_repository::Error {
    fn from(e: Error) -> Self {
        worker_repository::Error::Unknown(e.to_string())
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct FirestoreWorkerRepository {
    config: Config,
}

impl FirestoreWorkerRepository {
    const WORKERS: &'static str = "workers";

    pub fn new(config: Config) -> Self {
        Self { config }
    }

    async fn begin_transaction(&self) -> Result<FirestoreTransaction> {
        let (project_id, database_id) = (self.config.project_id(), self.config.database_id());
        let transaction =
            FirestoreTransaction::begin(project_id.to_owned(), database_id.to_owned()).await?;
        Ok(transaction)
    }

    async fn find_last_event_id(&self, worker_name: WorkerName) -> Result<Option<EventId>> {
        let transaction = self.begin_transaction().await?;
        let document = match transaction
            .get_document(Self::WORKERS, &worker_name.to_string())
            .await
        {
            Ok(None) => return Ok(None),
            Ok(Some(doc)) => Ok(doc),
            Err(e) => Err(e),
        }?;
        Ok(Some(EventId::from_str(get_field_as_str(
            &document, "event_id",
        )?)?))
    }

    async fn store_last_event_id(
        &self,
        worker_name: WorkerName,
        before: Option<EventId>,
        after: EventId,
    ) -> Result<()> {
        let transaction = self.begin_transaction().await?;
        let condition_type = match before {
            Some(_) => {
                let document = transaction
                    .get_document(Self::WORKERS, &worker_name.to_string())
                    .await?
                    .ok_or(Error::WorkerNotFound(worker_name))?;
                let update_time = document.update_time.expect("output contains update_time");
                ConditionType::UpdateTime(update_time)
            }
            None => ConditionType::Exists(false),
        };
        let document = Document {
            name: transaction.document_path(Self::WORKERS, &after.to_string()),
            fields: {
                let mut fields = HashMap::new();
                fields.insert("event_id".to_owned(), value_from_string(after.to_string()));
                fields
            },
            create_time: None,
            update_time: None,
        };
        transaction
            .push_write(Write {
                update_mask: None,
                update_transforms: vec![],
                current_document: Some(Precondition {
                    condition_type: Some(condition_type),
                }),
                operation: Some(Operation::Update(document)),
            })
            .await?;
        Ok(transaction.commit().await?)
    }
}

#[async_trait]
impl WorkerRepository for FirestoreWorkerRepository {
    async fn find_last_event_id(
        &self,
        worker_name: WorkerName,
    ) -> worker_repository::Result<Option<EventId>> {
        Ok(self.find_last_event_id(worker_name).await?)
    }

    async fn store_last_event_id(
        &self,
        worker_name: WorkerName,
        before: Option<EventId>,
        after: EventId,
    ) -> worker_repository::Result<()> {
        Ok(self.store_last_event_id(worker_name, before, after).await?)
    }
}
