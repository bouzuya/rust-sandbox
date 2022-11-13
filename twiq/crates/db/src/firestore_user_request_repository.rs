use std::{collections::HashMap, env, str::FromStr};

use crate::{
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
use domain::aggregate::{user::UserRequestId, user_request::UserRequest};
use event_store_core::{event_store::EventStore, EventStream, EventStreamId};
use use_case::user_request_repository::{self, UserRequestRepository};

struct FirestoreUserRequestRepository;

impl FirestoreUserRequestRepository {
    const USER_REQUEST_IDS: &'static str = "user_request_ids";
}

#[async_trait]
impl UserRequestRepository for FirestoreUserRequestRepository {
    async fn find(
        &self,
        id: UserRequestId,
    ) -> user_request_repository::Result<Option<UserRequest>> {
        // TODO: HasConfiguration trait
        // begin transaction & create event_store
        let project_id = env::var("PROJECT_ID")
            .map_err(|e| user_request_repository::Error::Unknown(e.to_string()))?;
        let database_id = "(default)".to_owned();
        let transaction = FirestoreTransaction::begin(project_id.clone(), database_id.clone())
            .await
            .map_err(|e| user_request_repository::Error::Unknown(e.to_string()))?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let document = match transaction
            .get_document(Self::USER_REQUEST_IDS, &id.to_string())
            .await
        {
            Ok(None) => return Ok(None),
            Ok(Some(doc)) => Ok(doc),
            Err(e) => Err(e),
        }
        .map_err(|e| user_request_repository::Error::Unknown(e.to_string()))?;
        let event_stream_id_as_str = get_field_as_str(&document, "event_stream_id").unwrap();
        let event_stream_id = EventStreamId::from_str(event_stream_id_as_str).unwrap();
        let event_stream = event_store
            .find_event_stream(event_stream_id)
            .await
            .unwrap();
        match event_stream {
            None => Ok(None),
            Some(event_stream) => UserRequest::try_from(event_stream)
                .map(Some)
                .map_err(|e| user_request_repository::Error::Unknown(e.to_string())),
        }
    }

    async fn store(
        &self,
        before: Option<UserRequest>,
        after: UserRequest,
    ) -> user_request_repository::Result<()> {
        // TODO: HasConfiguration trait
        // begin transaction & create event_store
        let project_id = env::var("PROJECT_ID")
            .map_err(|e| user_request_repository::Error::Unknown(e.to_string()))?;
        let database_id = "(default)".to_owned();
        let transaction = FirestoreTransaction::begin(project_id.clone(), database_id.clone())
            .await
            .map_err(|e| user_request_repository::Error::Unknown(e.to_string()))?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let user_request_id = after.id();
        let event_stream = EventStream::from(after);
        let event_stream_id = event_stream.id();

        let collection_id = Self::USER_REQUEST_IDS;
        let document_id = user_request_id.to_string();
        match before {
            Some(ref before_user_request) => {
                if before_user_request.id() != user_request_id {
                    return Err(user_request_repository::Error::Unknown(
                        "user_request_id not match".to_owned(),
                    ));
                }
                let document = transaction
                    .get_document(collection_id, &document_id)
                    .await
                    .map_err(|e| user_request_repository::Error::Unknown(e.to_string()))?
                    .ok_or_else(|| {
                        user_request_repository::Error::Unknown("not found".to_owned())
                    })?;
                let before_event_stream_id_as_str =
                    get_field_as_str(&document, "event_stream_id").unwrap();
                if before_event_stream_id_as_str != event_stream_id.to_string() {
                    return Err(user_request_repository::Error::Unknown(
                        "event_stream_id not match".to_owned(),
                    ));
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
                    .await
                    .map_err(|e| user_request_repository::Error::Unknown(e.to_string()))?;
            }
        }

        event_store
            .store(
                before.map(|aggregate| EventStream::from(aggregate).seq()),
                event_stream,
            )
            .await
            .map_err(|e| user_request_repository::Error::Unknown(e.to_string()))?;

        transaction
            .commit()
            .await
            .map_err(|e| user_request_repository::Error::Unknown(e.to_string()))?;
        Ok(())
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
        let repository = FirestoreUserRequestRepository;
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
