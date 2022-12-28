use std::collections::VecDeque;

use crate::{
    domain::ScheduledTweet,
    storage::{firestore::FirestoreStorage, Storage},
    token::Token,
};

pub struct TweetQueueStore {
    storage: FirestoreStorage,
}

impl TweetQueueStore {
    const DATABASE_ID: &str = "(default)";
    const COLLECTION_ID: &str = "twiq-light";
    const QUEUE_DOCUMENT_ID: &str = "queue";
    const TOKEN_DOCUMENT_ID: &str = "token";

    pub async fn new(
        project_id: String,
        google_application_credentials: Option<String>,
    ) -> anyhow::Result<Self> {
        let storage = FirestoreStorage::new(
            google_application_credentials,
            project_id,
            Self::DATABASE_ID.to_owned(),
            Self::COLLECTION_ID.to_owned(),
        )
        .await?;
        Ok(Self { storage })
    }

    pub async fn read_all(&self) -> anyhow::Result<VecDeque<ScheduledTweet>> {
        let data = self
            .storage
            .get_item(Self::QUEUE_DOCUMENT_ID.to_owned())
            .await?;
        Ok(match data {
            Some(d) => serde_json::from_str(&d)?,
            None => VecDeque::default(),
        })
    }

    pub async fn read_token(&self) -> anyhow::Result<Option<Token>> {
        let data = self
            .storage
            .get_item(Self::TOKEN_DOCUMENT_ID.to_owned())
            .await?;
        Ok(match data {
            Some(d) => Some(serde_json::from_str(&d)?),
            None => None,
        })
    }

    pub async fn write_token(&self, token: &Token) -> anyhow::Result<()> {
        self.storage
            .set_item(Self::TOKEN_DOCUMENT_ID.to_owned(), token.to_string())
            .await
    }

    pub async fn write_all(&self, data: &VecDeque<ScheduledTweet>) -> anyhow::Result<()> {
        let s = serde_json::to_string(&data)?;
        self.storage
            .set_item(Self::QUEUE_DOCUMENT_ID.to_owned(), s)
            .await
    }
}
