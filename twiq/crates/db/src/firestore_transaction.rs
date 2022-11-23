use std::sync::Arc;

use google_cloud_auth::Credential;
use tokio::sync::Mutex;
use tonic::{
    codegen::InterceptedService,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Code, Request, Status,
};

use crate::firestore_rpc::{
    google::firestore::v1::{
        firestore_client::FirestoreClient,
        transaction_options::{Mode, ReadWrite},
        BeginTransactionRequest, CommitRequest, Document, GetDocumentRequest, TransactionOptions,
        Write,
    },
    helper::{
        client, credential,
        path::{collection_path, database_path, document_path, documents_path},
        Result,
    },
};

#[derive(Clone)]
pub struct FirestoreTransaction {
    // TODO: アプリケーションごとの初期化で十分
    credential: Credential,
    project_id: String,
    database_id: String,
    // コマンド (トランザクション) ごとに初期化する (=FirestoreTransaction ごとで良い)
    channel: Channel,
    transaction: Vec<u8>,
    writes: Arc<Mutex<Vec<Write>>>,
    // リクエストごとに初期化する
    // - access_token: AccessToken
    // - client は access_token の設定のために await が必要になるが、
    //   tonic::service::interceptor::Intercepter は非同期に対応していないため、
    //   リクエストごとに初期化する必要がある
}

impl FirestoreTransaction {
    pub async fn begin(project_id: String, database_id: String) -> Result<Self> {
        let credential = credential().await?;
        let channel = Endpoint::from_static("https://firestore.googleapis.com")
            .tls_config(ClientTlsConfig::new().domain_name("firestore.googleapis.com"))?
            .connect()
            .await?;
        let mut client = client(&credential, channel.clone()).await?;
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
            channel,
            transaction,
            writes: Arc::new(Mutex::new(vec![])),
        })
    }

    pub async fn client(
        &self,
    ) -> Result<
        FirestoreClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    > {
        client(&self.credential, self.channel.clone()).await
    }

    pub async fn get_document(
        &self,
        collection_id: &str,
        document_id: &str,
    ) -> Result<Option<Document>> {
        self.client()
            .await?
            .get_document(GetDocumentRequest {
                name: self.document_path(collection_id, document_id),
                mask: None,
                // TODO
                consistency_selector: None,
                // Some(get_document_request::ConsistencySelector::Transaction(
                //     self.name(),
                // )),
            })
            .await
            .map(|response| Some(response.into_inner()))
            .or_else(|status| {
                if matches!(status.code(), Code::NotFound) {
                    Ok(None)
                } else {
                    Err(status)?
                }
            })
    }

    pub async fn commit(self) -> Result<()> {
        let writes = self.writes.lock().await.clone();
        let database = self.database_path();
        let mut client = client(&self.credential, self.channel.clone()).await?;
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

    pub fn documents_path(&self) -> String {
        documents_path(&self.project_id, &self.database_id)
    }

    pub fn project_id(&self) -> String {
        self.project_id.clone()
    }

    pub fn name(&self) -> Vec<u8> {
        self.transaction.clone()
    }

    pub async fn push_write(&self, write: Write) -> Result<()> {
        let mut writes = self.writes.lock().await;
        writes.push(write);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: test begin

    // TODO: test client

    // TODO: test commit

    // TODO: test colleciton_path

    // TODO: test database_id

    // TODO: test database_path

    // TODO: test document_path

    // TODO: test project_id

    // TODO: test name

    // TODO: test push_write
}
