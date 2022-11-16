use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    config::Config,
    firestore_rpc::{
        google::firestore::v1::{write::Operation, Document, Write},
        helper::{get_field_as_str, value_from_string},
    },
    firestore_transaction::FirestoreTransaction,
};
use query_handler::{user::User, user_store};

pub struct FirestoreUserStore {
    config: Config,
}

impl FirestoreUserStore {
    const QUERY_USERS: &'static str = "query_users";
    const QUERY_TWITTER_USER_IDS: &'static str = "query_twitter_user_ids";

    async fn begin_transaction(&self) -> user_store::Result<FirestoreTransaction> {
        let (project_id, database_id) = (self.config.project_id(), self.config.database_id());
        let transaction =
            FirestoreTransaction::begin(project_id.to_owned(), database_id.to_owned())
                .await
                .map_err(|e| user_store::Error::Unknown(e.to_string()))?;
        Ok(transaction)
    }
}

#[async_trait]
impl user_store::UserStore for FirestoreUserStore {
    async fn find_by_twitter_user_id(
        &self,
        twitter_user_id: &String,
    ) -> user_store::Result<Option<User>> {
        let transaction = self.begin_transaction().await?;

        // get user_id
        let document = match transaction
            .get_document(Self::QUERY_TWITTER_USER_IDS, twitter_user_id)
            .await
        {
            Ok(None) => return Ok(None),
            Ok(Some(doc)) => Ok(doc),
            Err(e) => Err(e),
        }
        .map_err(|e| user_store::Error::Unknown(e.to_string()))?;
        let user_id = get_field_as_str(&document, "user_id").expect("user_id not found");

        // get user
        let document = match transaction.get_document(Self::QUERY_USERS, user_id).await {
            Ok(None) => return Ok(None),
            Ok(Some(doc)) => Ok(doc),
            Err(e) => Err(e),
        }
        .map_err(|e| user_store::Error::Unknown(e.to_string()))?;
        let query_user = User {
            user_id: get_field_as_str(&document, "user_id")
                .expect("user_id not found")
                .to_owned(),
            twitter_user_id: get_field_as_str(&document, "twitter_user_id")
                .expect("twitter_user_id not found")
                .to_owned(),
            twitter_user_name: get_field_as_str(&document, "twitter_user_name")
                .expect("twitter_user_name not found")
                .to_owned(),
        };
        Ok(Some(query_user))
    }

    async fn store(&self, _before: Option<User>, after: User) -> user_store::Result<()> {
        let transaction = self.begin_transaction().await?;

        // store query_twitter_user_id
        let document = Document {
            name: transaction.document_path(Self::QUERY_TWITTER_USER_IDS, &after.twitter_user_id),
            fields: {
                let mut fields = HashMap::new();
                fields.insert(
                    "user_id".to_owned(),
                    value_from_string(after.user_id.clone()),
                );
                fields
            },
            create_time: None,
            update_time: None,
        };
        transaction
            .push_write(Write {
                update_mask: None,
                update_transforms: vec![],
                current_document: None,
                operation: Some(Operation::Update(document)),
            })
            .await
            .map_err(|e| user_store::Error::Unknown(e.to_string()))?;

        // store query_user
        let document = Document {
            name: transaction.document_path(Self::QUERY_USERS, &after.user_id),
            fields: {
                let mut fields = HashMap::new();
                fields.insert("user_id".to_owned(), value_from_string(after.user_id));
                fields.insert(
                    "twitter_user_id".to_owned(),
                    value_from_string(after.twitter_user_id),
                );
                fields.insert(
                    "twitter_user_name".to_owned(),
                    value_from_string(after.twitter_user_name),
                );
                fields
            },
            create_time: None,
            update_time: None,
        };
        transaction
            .push_write(Write {
                update_mask: None,
                update_transforms: vec![],
                current_document: None,
                operation: Some(Operation::Update(document)),
            })
            .await
            .map_err(|e| user_store::Error::Unknown(e.to_string()))?;

        transaction
            .commit()
            .await
            .map_err(|e| user_store::Error::Unknown(e.to_string()))?;
        Ok(())
    }
}
