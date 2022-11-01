use std::sync::Arc;

use google_cloud_auth::Credential;
use tokio::sync::Mutex;

use crate::firestore_rpc::{
    google::firestore::v1::{
        transaction_options::{Mode, ReadWrite},
        BeginTransactionRequest, CommitRequest, TransactionOptions, Write,
    },
    helper::{
        client, credential,
        path::{collection_path, database_path, document_path},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Error(#[from] crate::firestore_rpc::helper::Error),
    #[error("status {0}")]
    Status(#[from] tonic::Status),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct FirestoreTransaction {
    credential: Credential,
    project_id: String,
    database_id: String,
    transaction: Vec<u8>,
    writes: Arc<Mutex<Vec<Write>>>,
}

impl FirestoreTransaction {
    pub async fn begin(project_id: String, database_id: String) -> Result<Self> {
        let credential = credential().await?;
        let mut client = client(&credential).await?;
        let database = database_path(&project_id, &database_id);
        let response = client
            .begin_transaction(BeginTransactionRequest {
                database,
                options: Some(TransactionOptions {
                    mode: Some(Mode::ReadWrite(ReadWrite {
                        retry_transaction: vec![],
                    })),
                }),
            })
            .await?;
        let transaction = response.into_inner().transaction;
        Ok(Self {
            credential,
            project_id,
            database_id,
            transaction,
            writes: Arc::new(Mutex::new(vec![])),
        })
    }

    pub async fn commit(self) -> Result<()> {
        let writes = self.writes.lock().await.clone();
        let database = self.database_path();
        let mut client = client(&self.credential).await?;
        let _ = client
            .commit(CommitRequest {
                database,
                writes,
                transaction: self.transaction,
            })
            .await?;
        Ok(())
    }

    pub fn collection_path(&self, collection_id: &str) -> String {
        collection_path(&self.project_id, &self.database_id, collection_id)
    }

    pub fn database_id(&self) -> String {
        self.database_id.clone()
    }

    pub fn database_path(&self) -> String {
        database_path(&self.project_id, &self.database_id)
    }

    pub fn document_path(&self, collection_id: &str, document_id: &str) -> String {
        document_path(
            &self.project_id,
            &self.database_id,
            collection_id,
            document_id,
        )
    }

    pub fn project_id(&self) -> String {
        self.project_id.clone()
    }
}

#[cfg(test)]
mod tests {
    // TODO: test begin

    // TODO: test commit

    // TODO: test colleciton_path

    // TODO: test database_id

    // TODO: test database_path

    // TODO: test document_path

    // TODO: test project_id
}
