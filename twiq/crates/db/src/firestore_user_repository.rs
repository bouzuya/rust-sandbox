use std::{collections::HashMap, env, str::FromStr};

use async_trait::async_trait;
use domain::aggregate::user::{TwitterUserId, User, UserId};
use event_store_core::{event_store::EventStore, EventStream, EventStreamId};
use use_case::user_repository::{self, UserRepository};

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

struct FirestoreUserRepository;

#[async_trait]
impl UserRepository for FirestoreUserRepository {
    async fn find(&self, id: UserId) -> user_repository::Result<Option<User>> {
        // TODO: HasConfiguration trait
        // begin transaction & create event_store
        let project_id =
            env::var("PROJECT_ID").map_err(|e| user_repository::Error::Unknown(e.to_string()))?;
        let database_id = "(default)".to_owned();
        let transaction = FirestoreTransaction::begin(project_id.clone(), database_id.clone())
            .await
            .map_err(|e| user_repository::Error::Unknown(e.to_string()))?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let document = transaction
            .get_document("user_ids", &id.to_string())
            .await
            .unwrap();
        let event_stream_id_as_str = get_field_as_str(&document, "event_stream_id").unwrap();
        let event_stream_id = EventStreamId::from_str(event_stream_id_as_str).unwrap();
        let event_stream = event_store
            .find_event_stream(event_stream_id)
            .await
            .unwrap();
        match event_stream {
            None => Ok(None),
            Some(event_stream) => User::try_from(event_stream)
                .map(Some)
                .map_err(|e| user_repository::Error::Unknown(e.to_string())),
        }
    }

    async fn find_by_twitter_user_id(
        &self,
        _twitter_user_id: &TwitterUserId,
    ) -> user_repository::Result<Option<User>> {
        todo!()
    }

    async fn store(&self, before: Option<User>, after: User) -> user_repository::Result<()> {
        // TODO: HasConfiguration trait
        // begin transaction & create event_store
        let project_id =
            env::var("PROJECT_ID").map_err(|e| user_repository::Error::Unknown(e.to_string()))?;
        let database_id = "(default)".to_owned();
        let transaction = FirestoreTransaction::begin(project_id.clone(), database_id.clone())
            .await
            .map_err(|e| user_repository::Error::Unknown(e.to_string()))?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let twitter_user_id = after.twitter_user_id().clone();
        let user_id = after.id();
        let event_stream = EventStream::from(after);
        let event_stream_id = event_stream.id();

        let collection_id = "user_ids";
        let document_id = user_id.to_string();
        match before {
            Some(ref before_user) => {
                if before_user.id() != user_id {
                    return Err(user_repository::Error::Unknown(
                        "user_id not match".to_owned(),
                    ));
                }
                let document = transaction
                    .get_document(collection_id, &document_id)
                    .await
                    .map_err(|e| user_repository::Error::Unknown(e.to_string()))?;
                let before_event_stream_id_as_str =
                    get_field_as_str(&document, "event_stream_id").unwrap();
                if before_event_stream_id_as_str != event_stream_id.to_string() {
                    return Err(user_repository::Error::Unknown(
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
                    .map_err(|e| user_repository::Error::Unknown(e.to_string()))?;
            }
        }

        let collection_id = "twitter_user_ids";
        let document_id = twitter_user_id.to_string();
        match before {
            Some(ref before_user) => {
                if before_user.twitter_user_id() != &twitter_user_id {
                    return Err(user_repository::Error::Unknown(
                        "twitter_user_id not match".to_owned(),
                    ));
                }
                let document = transaction
                    .get_document(collection_id, &document_id)
                    .await
                    .map_err(|e| user_repository::Error::Unknown(e.to_string()))?;
                let before_user_id_as_str = get_field_as_str(&document, "user_id").unwrap();
                if before_user_id_as_str != user_id.to_string() {
                    return Err(user_repository::Error::Unknown(
                        "user_id not match".to_owned(),
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
                                map.insert(
                                    "twitter_user_id".to_owned(),
                                    value_from_string(twitter_user_id.to_string()),
                                );
                                map.insert(
                                    "user_id".to_owned(),
                                    value_from_string(user_id.to_string()),
                                );
                                map
                            },
                            create_time: None,
                            update_time: None,
                        })),
                    })
                    .await
                    .map_err(|e| user_repository::Error::Unknown(e.to_string()))?;
            }
        };

        event_store
            .store(
                before.map(|aggregate| EventStream::from(aggregate).seq()),
                event_stream,
            )
            .await
            .map_err(|e| user_repository::Error::Unknown(e.to_string()))?;

        transaction
            .commit()
            .await
            .map_err(|e| user_repository::Error::Unknown(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use domain::aggregate::user::{At, TwitterUserName};

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test() -> anyhow::Result<()> {
        let user = User::create(TwitterUserId::from_str("125962981")?)?;
        let repository = FirestoreUserRepository;
        // TODO: support "Not Found"
        // assert!(repository.find(user.id()).await?.is_none());
        repository.store(None, user.clone()).await?;
        assert_eq!(repository.find(user.id()).await?, Some(user.clone()));
        let updated = user.update(TwitterUserName::from_str("bouzuya")?, At::now())?;
        repository
            .store(Some(user.clone()), updated.clone())
            .await?;
        assert_eq!(repository.find(user.id()).await?, Some(updated));
        Ok(())
    }
}
