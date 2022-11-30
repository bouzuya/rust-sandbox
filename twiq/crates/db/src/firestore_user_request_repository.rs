use std::{collections::HashMap, str::FromStr};

use crate::{
    config::Config,
    firestore_rpc::{
        google::firestore::v1::{
            precondition::ConditionType, write::Operation, Document, Precondition, Write,
        },
        helper::{get_field_as_str, value_from_string},
    },
    firestore_rpc_event_store::FirestoreRpcEventStore,
    firestore_transaction::FirestoreTransaction,
};
use async_trait::async_trait;
use command_handler::user_request_repository::{self, UserRequestRepository};
use domain::aggregate::{user::UserRequestId, user_request::UserRequest};
use event_store_core::{event_store::EventStore, EventStream, EventStreamId};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("event_store_core::event_store {0}")]
    EventStore(#[from] event_store_core::event_store::Error),
    #[error("event_store_core::event_stream_id {0}")]
    EventStreamId(#[from] event_store_core::event_stream_id::Error),
    #[error("firestore_rpc::helper {0}")]
    FirestoreRpcHelper(#[from] crate::firestore_rpc::helper::Error),
    #[error("firestore_rpc::helper::GetFieldError {0}")]
    FirestoreRpcHelperGetField(#[from] crate::firestore_rpc::helper::GetFieldError),
    #[error("domain::aggregate::user_request {0}")]
    UserRequest(#[from] domain::aggregate::user_request::Error),
    #[error("user_request_id not found {0}")]
    UserRequestIdNotFound(UserRequestId),
    #[error("unknown {0}")]
    Unknown(String),
}

impl From<Error> for user_request_repository::Error {
    fn from(e: Error) -> Self {
        user_request_repository::Error::Unknown(e.to_string())
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct FirestoreUserRequestRepository {
    config: Config,
}

impl FirestoreUserRequestRepository {
    const USER_REQUEST_IDS: &'static str = "user_request_ids";

    pub fn new(config: Config) -> Self {
        Self { config }
    }

    async fn find(&self, id: UserRequestId) -> Result<Option<UserRequest>> {
        // begin transaction & create event_store
        let (project_id, database_id) = (self.config.project_id(), self.config.database_id());
        let transaction =
            FirestoreTransaction::begin(project_id.to_owned(), database_id.to_owned()).await?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let document = match transaction
            .get_document(Self::USER_REQUEST_IDS, &id.to_string())
            .await
        {
            Ok(None) => return Ok(None),
            Ok(Some(doc)) => Ok(doc),
            Err(e) => Err(e),
        }?;
        let event_stream_id_as_str = get_field_as_str(&document, "event_stream_id")?;
        let event_stream_id = EventStreamId::from_str(event_stream_id_as_str)?;
        let event_stream = event_store.find_event_stream(event_stream_id).await?;
        match event_stream {
            None => Ok(None),
            Some(event_stream) => Ok(UserRequest::try_from(event_stream).map(Some)?),
        }
    }

    async fn store(&self, before: Option<UserRequest>, after: UserRequest) -> Result<()> {
        // begin transaction & create event_store
        let (project_id, database_id) = (self.config.project_id(), self.config.database_id());
        let transaction =
            FirestoreTransaction::begin(project_id.to_owned(), database_id.to_owned()).await?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let user_request_id = after.id();
        let event_stream = EventStream::from(after);
        let event_stream_id = event_stream.id();

        let collection_id = Self::USER_REQUEST_IDS;
        let document_id = user_request_id.to_string();
        match before {
            Some(ref before_user_request) => {
                if before_user_request.id() != user_request_id {
                    return Err(Error::Unknown("user_request_id not match".to_owned()));
                }
                let document = transaction
                    .get_document(collection_id, &document_id)
                    .await?
                    .ok_or(Error::UserRequestIdNotFound(user_request_id))?;
                let before_event_stream_id_as_str = get_field_as_str(&document, "event_stream_id")?;
                if before_event_stream_id_as_str != event_stream_id.to_string() {
                    return Err(Error::Unknown("event_stream_id not match".to_owned()));
                }
            }
            None => {
                transaction
                    .push_write(Write {
                        update_mask: None,
                        update_transforms: vec![],
                        current_document: Some(Precondition {
                            condition_type: Some(ConditionType::Exists(false)),
                        }),
                        operation: Some(Operation::Update(Document {
                            name: transaction.document_path(collection_id, &document_id),
                            fields: {
                                let mut map = HashMap::new();
                                map.insert("user_id".to_owned(), value_from_string(document_id));
                                map.insert(
                                    "event_stream_id".to_owned(),
                                    value_from_string(event_stream_id.to_string()),
                                );
                                map
                            },
                            create_time: None,
                            update_time: None,
                        })),
                    })
                    .await?;
            }
        }

        event_store
            .store(
                before.map(|aggregate| EventStream::from(aggregate).seq()),
                event_stream,
            )
            .await?;

        transaction.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl UserRequestRepository for FirestoreUserRequestRepository {
    async fn find(
        &self,
        id: UserRequestId,
    ) -> user_request_repository::Result<Option<UserRequest>> {
        Ok(self.find(id).await?)
    }

    async fn store(
        &self,
        before: Option<UserRequest>,
        after: UserRequest,
    ) -> user_request_repository::Result<()> {
        Ok(self.store(before, after).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use domain::aggregate::user::{TwitterUserId, UserId};

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test() -> anyhow::Result<()> {
        let user_id = UserId::generate();
        let id = UserRequestId::generate();
        let twitter_user_id = TwitterUserId::from_str("125962981")?;
        let user_request = UserRequest::create(id, twitter_user_id, user_id)?;
        let config = Config::load_from_env();
        let repository = FirestoreUserRequestRepository { config };
        assert!(repository.find(user_request.id()).await?.is_none());
        repository.store(None, user_request.clone()).await?;
        assert_eq!(
            repository.find(user_request.id()).await?,
            Some(user_request.clone())
        );
        let started = user_request.start()?;
        repository
            .store(Some(user_request.clone()), started.clone())
            .await?;
        assert_eq!(
            repository.find(user_request.id()).await?,
            Some(started.clone())
        );

        // store twice
        assert!(repository
            .store(Some(user_request.clone()), started.clone())
            .await
            .is_err());

        // duplicate id
        let user_id = UserId::generate();
        let twitter_user_id = TwitterUserId::from_str("125962981")?;
        let user_request2 = UserRequest::create(id, twitter_user_id, user_id)?;
        assert!(repository.store(None, user_request2.clone()).await.is_err());

        Ok(())
    }
}
