use std::sync::Arc;

use google_cloud_auth::{Credential, CredentialConfig};
use tokio::sync::Mutex;
use tonic::{
    codegen::InterceptedService,
    metadata::AsciiMetadataValue,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Request, Status,
};

use crate::firestore_rpc::{
    google::firestore::v1::{
        firestore_client::FirestoreClient,
        transaction_options::{Mode, ReadWrite},
        BeginTransactionRequest, CommitRequest, TransactionOptions, Write,
    },
    helper::path::{collection_path, database_path, document_path},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("google_cloud_auth {0}")]
    GoogleCloudAuth(#[from] google_cloud_auth::Error),
    #[error("status {0}")]
    Status(#[from] Status),
    #[error("tonic invalid metadata value {0}")]
    TonicInvalidMetadataValue(#[from] tonic::metadata::errors::InvalidMetadataValue),
    #[error("tonic transport {0}")]
    TonicTransport(#[from] tonic::transport::Error),
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
        let credential = Self::credential().await?;
        let mut client = Self::client(&credential).await?;
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
        let mut client = Self::client(&self.credential).await?;
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

    async fn client(
        credential: &Credential,
    ) -> Result<
        FirestoreClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    > {
        let access_token = credential.access_token().await?;
        let channel = Endpoint::from_static("https://firestore.googleapis.com")
            .tls_config(ClientTlsConfig::new().domain_name("firestore.googleapis.com"))?
            .connect()
            .await?;
        let mut metadata_value =
            AsciiMetadataValue::try_from(format!("Bearer {}", access_token.value))?;
        metadata_value.set_sensitive(true);
        let client = FirestoreClient::with_interceptor(channel, move |mut request: Request<()>| {
            request
                .metadata_mut()
                .insert("authorization", metadata_value.clone());
            Ok(request)
        });
        Ok(client)
    }

    async fn credential() -> Result<Credential> {
        let config = CredentialConfig::builder()
            .scopes(vec!["https://www.googleapis.com/auth/cloud-platform".into()])
            .build()?;
        Ok(Credential::find_default(config).await?)
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
