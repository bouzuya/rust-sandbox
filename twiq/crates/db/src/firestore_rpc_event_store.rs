use async_trait::async_trait;
use event_store_core::{
    event_store::EventStore, Event, EventId, EventStream, EventStreamId, EventStreamSeq,
};
use google_cloud_auth::Credential;
use tonic::{
    codegen::InterceptedService,
    metadata::AsciiMetadataValue,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Request, Status,
};

use crate::firestore_rpc::google::firestore::v1::{
    firestore_client::FirestoreClient,
    transaction_options::{Mode, ReadWrite},
    BeginTransactionRequest, TransactionOptions,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("google_cloud_auth {0}")]
    GoogleCloudAuth(#[from] google_cloud_auth::Error),
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

struct FirestoreRpcEventStore {
    project_id: String,
    database_id: String,
    credential: Credential,
    transaction: Vec<u8>,
}

impl FirestoreRpcEventStore {
    pub async fn begin_transaction(
        project_id: String,
        database_id: String,
        credential: Credential,
    ) -> Result<Self> {
        let mut client = Self::client(&credential)
            .await
            .map_err(|status| Error::Unknown(status.to_string()))?;
        let database = format!("projects/{}/databases/{}", project_id, database_id);
        let response = client
            .begin_transaction(BeginTransactionRequest {
                database,
                options: Some(TransactionOptions {
                    mode: Some(Mode::ReadWrite(ReadWrite {
                        retry_transaction: vec![],
                    })),
                }),
            })
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let transaction = response.into_inner().transaction;
        Ok(Self {
            project_id,
            database_id,
            credential,
            transaction,
        })
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
            .tls_config(ClientTlsConfig::new().domain_name("firestore.googleapis.com"))
            .map_err(|e| Error::Unknown(e.to_string()))?
            .connect()
            .await
            .map_err(|status| Error::Unknown(status.to_string()))?;
        let mut metadata_value =
            AsciiMetadataValue::try_from(format!("Bearer {}", access_token.value))
                .map_err(|e| Error::Unknown(e.to_string()))?;
        metadata_value.set_sensitive(true);
        let client = FirestoreClient::with_interceptor(channel, move |mut request: Request<()>| {
            request
                .metadata_mut()
                .insert("authorization", metadata_value.clone());
            Ok(request)
        });
        Ok(client)
    }
}

#[async_trait]
impl EventStore for FirestoreRpcEventStore {
    async fn find_event(
        &self,
        event_id: EventId,
    ) -> event_store_core::event_store::Result<Option<Event>> {
        todo!()
    }

    async fn find_event_ids(
        &self,
        after: Option<EventId>,
    ) -> event_store_core::event_store::Result<Vec<EventId>> {
        todo!()
    }

    async fn find_event_stream(
        &self,
        event_stream_id: EventStreamId,
    ) -> event_store_core::event_store::Result<Option<EventStream>> {
        todo!()
    }

    async fn find_events(
        &self,
        after: Option<EventId>,
    ) -> event_store_core::event_store::Result<Vec<Event>> {
        todo!()
    }

    async fn store(
        &self,
        current: Option<EventStreamSeq>,
        event_stream: EventStream,
    ) -> event_store_core::event_store::Result<()> {
        todo!()
    }
}
