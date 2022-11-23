use std::{collections::HashMap, str::FromStr};

use async_trait::async_trait;
use domain::aggregate::user::{TwitterUserId, User, UserId};
use event_store_core::{event_store::EventStore, EventStream, EventStreamId};
use use_case::user_repository::{self, UserRepository};

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

pub struct FirestoreUserRepository {
    config: Config,
}

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
    #[error("domain::aggregate::user {0}")]
    User(#[from] domain::aggregate::user::Error),
    #[error("unknown {0}")]
    Unknown(String),
}

impl From<Error> for user_repository::Error {
    fn from(e: Error) -> Self {
        user_repository::Error::Unknown(e.to_string())
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl FirestoreUserRepository {
    const USER_IDS: &'static str = "user_ids";
    const TWITTER_USER_IDS: &'static str = "twitter_user_ids";

    pub fn new(config: Config) -> Self {
        Self { config }
    }

    async fn find(&self, id: UserId) -> Result<Option<User>> {
        // begin transaction & create event_store
        let (project_id, database_id) = (self.config.project_id(), self.config.database_id());
        let transaction =
            FirestoreTransaction::begin(project_id.to_owned(), database_id.to_owned()).await?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let document = match transaction
            .get_document(Self::USER_IDS, &id.to_string())
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
            Some(event_stream) => Ok(User::try_from(event_stream).map(Some)?),
        }
    }

    async fn find_by_twitter_user_id(
        &self,
        twitter_user_id: &TwitterUserId,
    ) -> Result<Option<User>> {
        // begin transaction & create event_store
        let (project_id, database_id) = (self.config.project_id(), self.config.database_id());
        let transaction =
            FirestoreTransaction::begin(project_id.to_owned(), database_id.to_owned()).await?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let document = match transaction
            .get_document(Self::TWITTER_USER_IDS, &twitter_user_id.to_string())
            .await
        {
            Ok(None) => return Ok(None),
            Ok(Some(doc)) => Ok(doc),
            Err(e) => Err(e),
        }?;
        let user_id_as_str = get_field_as_str(&document, "user_id")?;

        let document = match transaction
            .get_document(Self::USER_IDS, user_id_as_str)
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
            Some(event_stream) => Ok(User::try_from(event_stream).map(Some)?),
        }
    }

    async fn store(&self, before: Option<User>, after: User) -> Result<()> {
        // begin transaction & create event_store
        let (project_id, database_id) = (self.config.project_id(), self.config.database_id());
        let transaction =
            FirestoreTransaction::begin(project_id.to_owned(), database_id.to_owned()).await?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let twitter_user_id = after.twitter_user_id().clone();
        let user_id = after.id();
        let event_stream = EventStream::from(after);
        let event_stream_id = event_stream.id();

        let collection_id = Self::USER_IDS;
        let document_id = user_id.to_string();
        match before {
            Some(ref before_user) => {
                if before_user.id() != user_id {
                    return Err(Error::Unknown("user_id not match".to_owned()));
                }
                let document = transaction
                    .get_document(collection_id, &document_id)
                    .await?
                    .ok_or_else(|| Error::Unknown("not found".to_owned()))?;
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

        let collection_id = Self::TWITTER_USER_IDS;
        let document_id = twitter_user_id.to_string();
        match before {
            Some(ref before_user) => {
                if before_user.twitter_user_id() != &twitter_user_id {
                    return Err(Error::Unknown("twitter_user_id not match".to_owned()));
                }
                let document = transaction
                    .get_document(collection_id, &document_id)
                    .await?
                    .ok_or_else(|| Error::Unknown("not found".to_owned()))?;
                let before_user_id_as_str = get_field_as_str(&document, "user_id")?;
                if before_user_id_as_str != user_id.to_string() {
                    return Err(Error::Unknown("user_id not match".to_owned()));
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
                    .await?;
            }
        };

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
impl UserRepository for FirestoreUserRepository {
    async fn find(&self, id: UserId) -> user_repository::Result<Option<User>> {
        Ok(self.find(id).await?)
    }

    async fn find_by_twitter_user_id(
        &self,
        twitter_user_id: &TwitterUserId,
    ) -> user_repository::Result<Option<User>> {
        Ok(self.find_by_twitter_user_id(twitter_user_id).await?)
    }

    async fn store(&self, before: Option<User>, after: User) -> user_repository::Result<()> {
        Ok(self.store(before, after).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use domain::aggregate::user::TwitterUserName;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test() -> anyhow::Result<()> {
        let user = User::create(TwitterUserId::from_str("125962981")?)?;
        let config = Config::load_from_env();
        let repository = FirestoreUserRepository { config };
        assert!(repository.find(user.id()).await?.is_none());
        repository.store(None, user.clone()).await?;
        assert_eq!(repository.find(user.id()).await?, Some(user.clone()));
        let updated = user.update(TwitterUserName::from_str("bouzuya")?)?;
        repository
            .store(Some(user.clone()), updated.clone())
            .await?;
        assert_eq!(repository.find(user.id()).await?, Some(updated.clone()));
        assert_eq!(
            repository
                .find_by_twitter_user_id(user.twitter_user_id())
                .await?,
            Some(updated)
        );
        Ok(())
    }
}
