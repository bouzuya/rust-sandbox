use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_trait::async_trait;
use event_store_core::{
    event_store::{self, EventStore},
    Event, EventId, EventPayload, EventStream, EventStreamId, EventStreamSeq, EventType,
};
use google_cloud_auth::Credential;
use prost_types::Timestamp;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tokio::sync::Mutex;
use tonic::{
    codegen::InterceptedService,
    metadata::AsciiMetadataValue,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Request, Status,
};

use crate::firestore_rpc::{
    google::firestore::v1::{
        document_transform::{
            field_transform::{ServerValue, TransformType},
            FieldTransform,
        },
        firestore_client::FirestoreClient,
        get_document_request,
        precondition::ConditionType,
        run_query_request::{self, QueryType},
        structured_query::{
            field_filter, filter::FilterType, CollectionSelector, Direction, FieldFilter,
            FieldReference, Filter, Order, Projection,
        },
        transaction_options::{Mode, ReadOnly, ReadWrite},
        write::Operation,
        BeginTransactionRequest, CommitRequest, Document, GetDocumentRequest, Precondition,
        RunQueryRequest, StructuredQuery, TransactionOptions, Value, Write,
    },
    helper::{
        get_field_as_i64, get_field_as_str, get_field_as_timestamp, value_from_i64,
        value_from_string, value_from_timestamp,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("google_cloud_auth {0}")]
    GoogleCloudAuth(#[from] google_cloud_auth::Error),
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct FirestoreRpcEventStore {
    credential: Credential,
    project_id: String,
    database_id: String,
    transaction: Vec<u8>,
    writes: Arc<Mutex<Vec<Write>>>,
}

impl FirestoreRpcEventStore {
    pub fn new(
        credential: Credential,
        project_id: String,
        database_id: String,
        transaction: Vec<u8>,
    ) -> Self {
        Self {
            credential,
            project_id,
            database_id,
            transaction,
            writes: Arc::new(Mutex::new(vec![])),
        }
    }

    // TODO: extract
    pub async fn begin_transaction(
        credential: &Credential,
        project_id: &str,
        database_id: &str,
    ) -> Result<Vec<u8>> {
        let mut client = Self::client(credential)
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
        Ok(response.into_inner().transaction)
    }

    // TODO: extract
    pub async fn commit(
        credential: &Credential,
        project_id: &str,
        database_id: &str,
        transaction: Vec<u8>,
        writes: Vec<Write>,
    ) -> Result<()> {
        let database = format!("projects/{}/databases/{}", project_id, database_id);
        let mut client = Self::client(credential).await?;
        let _ = client
            .commit(CommitRequest {
                database,
                writes,
                transaction,
            })
            .await
            .map_err(|status| Error::Unknown(status.to_string()))?;
        Ok(())
    }

    pub async fn writes(&self) -> Vec<Write> {
        let writes = self.writes.lock().await;
        writes.clone()
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
    async fn find_event(&self, event_id: EventId) -> event_store::Result<Option<Event>> {
        let mut client = Self::client(&self.credential)
            .await
            .map_err(|status| event_store::Error::Unknown(status.to_string()))?;
        let collection_id = "events";
        let document_id = event_id.to_string();
        let name = format!(
            "projects/{}/databases/{}/documents/{}/{}",
            self.project_id, self.database_id, collection_id, document_id
        );
        let response = client
            .get_document(GetDocumentRequest {
                name,
                mask: None,
                consistency_selector: Some(get_document_request::ConsistencySelector::Transaction(
                    self.transaction.clone(),
                )),
            })
            .await
            .map_err(|e| event_store::Error::Unknown(e.to_string()))?;
        let document = response.into_inner();
        // FIXME: error handling
        let event = event_from_fields(&document).unwrap();
        Ok(Some(event))
    }

    async fn find_event_ids(&self, after: Option<EventId>) -> event_store::Result<Vec<EventId>> {
        let event_id = match after {
            Some(a) => a,
            None => todo!(),
        };

        let mut client = Self::client(&self.credential)
            .await
            .map_err(|status| event_store::Error::Unknown(status.to_string()))?;

        // get requested_at
        let requested_at = {
            let collection_id = "events";
            let document_id = event_id.to_string();
            let document_path = format!("{}/{}", collection_id, document_id);
            let name = format!(
                "projects/{}/databases/{}/documents/{}",
                self.project_id, self.database_id, document_path
            );
            let response = client
                .get_document(GetDocumentRequest {
                    name,
                    mask: None,
                    consistency_selector: Some(
                        get_document_request::ConsistencySelector::Transaction(
                            self.transaction.clone(),
                        ),
                    ),
                })
                .await
                .map_err(|status| event_store::Error::Unknown(status.to_string()))?;
            let document = response.into_inner();
            get_field_as_timestamp(&document, "requested_at").unwrap()
        };

        todo!()
    }

    async fn find_event_stream(
        &self,
        event_stream_id: EventStreamId,
    ) -> event_store::Result<Option<EventStream>> {
        todo!()
    }

    async fn find_events(&self, after: Option<EventId>) -> event_store::Result<Vec<Event>> {
        todo!()
    }

    async fn store(
        &self,
        current: Option<EventStreamSeq>,
        event_stream: EventStream,
    ) -> event_store::Result<()> {
        let mut client = Self::client(&self.credential)
            .await
            .map_err(|status| event_store::Error::Unknown(status.to_string()))?;
        let collection_id = "event_streams";
        let document_id = event_stream.id().to_string();
        let event_stream_document = Document {
            name: format!(
                "projects/{}/databases/{}/documents/{}/{}",
                self.project_id, self.database_id, collection_id, document_id
            ),
            fields: event_stream_to_fields(event_stream.id(), event_stream.seq()),
            create_time: None,
            update_time: None,
        };
        let precondition = match current {
            Some(expected_event_stream_seq) => {
                let (_, event_stream_seq, update_time) = get_event_stream(
                    &mut client,
                    &self.project_id,
                    self.transaction.clone(),
                    &self.database_id,
                    event_stream.id(),
                )
                .await
                .map_err(|e| event_store::Error::Unknown(e.to_string()))?;
                if event_stream_seq != expected_event_stream_seq {
                    return Err(event_store::Error::Unknown("conflict".to_owned()));
                }
                Precondition {
                    condition_type: Some(ConditionType::UpdateTime(update_time)),
                }
            }
            None => Precondition {
                condition_type: Some(ConditionType::Exists(false)),
            },
        };
        let mut writes = self.writes.lock().await;
        writes.push(Write {
            update_mask: None,
            update_transforms: vec![],
            current_document: Some(precondition),
            operation: Some(Operation::Update(event_stream_document)),
        });

        for event in event_stream.events() {
            let collection_id = "events";
            let document_id = event.id().to_string();
            writes.push(Write {
                update_mask: None,
                update_transforms: vec![FieldTransform {
                    field_path: "requested_at".to_owned(),
                    transform_type: Some(TransformType::SetToServerValue(
                        ServerValue::RequestTime as i32,
                    )),
                }],
                current_document: Some(Precondition {
                    condition_type: Some(ConditionType::Exists(false)),
                }),
                operation: Some(Operation::Update(Document {
                    name: format!(
                        "projects/{}/databases/{}/documents/{}/{}",
                        &self.project_id, &self.database_id, collection_id, document_id
                    ),
                    fields: event_to_fields(&event),
                    create_time: None,
                    update_time: None,
                })),
            });
        }
        Ok(())
    }
}

fn event_stream_to_fields(
    event_stream_id: EventStreamId,
    event_stream_seq: EventStreamSeq,
) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    map.insert(
        "id".to_owned(),
        value_from_string(event_stream_id.to_string()),
    );
    map.insert(
        "seq".to_owned(),
        value_from_i64(i64::from(event_stream_seq)),
    );
    map
}

fn event_from_fields(document: &Document) -> Result<Event> {
    // FIXME: error handling
    let id = EventId::from_str(get_field_as_str(document, "id").unwrap()).unwrap();
    let r#type = EventType::from_str(get_field_as_str(document, "type").unwrap()).unwrap();
    let stream_id =
        EventStreamId::from_str(get_field_as_str(document, "stream_id").unwrap()).unwrap();
    let stream_seq =
        EventStreamSeq::try_from(get_field_as_i64(document, "stream_seq").unwrap()).unwrap();
    let payload = EventPayload::from_str(get_field_as_str(document, "data").unwrap()).unwrap();
    Ok(Event::new(id, r#type, stream_id, stream_seq, payload))
}

fn event_to_fields(event: &Event) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    map.insert("id".to_owned(), value_from_string(event.id().to_string()));
    map.insert(
        "type".to_owned(),
        value_from_string(event.r#type().to_string()),
    );
    map.insert(
        "stream_id".to_owned(),
        value_from_string(event.stream_id().to_string()),
    );
    map.insert(
        "stream_seq".to_owned(),
        value_from_i64(i64::from(event.stream_seq())),
    );
    map.insert(
        "data".to_owned(),
        value_from_string(event.payload().to_string()),
    );
    map
}

async fn get_event_stream(
    client: &mut FirestoreClient<
        InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
    >,
    project_id: &str,
    transaction: Vec<u8>,
    database_id: &str,
    event_stream_id: EventStreamId,
) -> Result<(EventStreamId, EventStreamSeq, Timestamp), Error> {
    let collection_id = "event_streams";
    let document_id = event_stream_id.to_string();
    let name = format!(
        "projects/{}/databases/{}/documents/{}/{}",
        project_id, database_id, collection_id, document_id
    );
    let response = client
        .get_document(GetDocumentRequest {
            name,
            mask: None,
            consistency_selector: Some(get_document_request::ConsistencySelector::Transaction(
                transaction,
            )),
        })
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    // TODO: check status_code

    let document = response.into_inner();
    let event_stream_id = get_field_as_str(&document, "id")
        .map(EventStreamId::from_str)
        .ok_or_else(|| Error::Unknown("id field is not found".to_owned()))?
        .map_err(|_| Error::Unknown("id field can't be converted to EventStreamId".to_owned()))?;
    let event_stream_seq = get_field_as_i64(&document, "seq")
        .map(EventStreamSeq::try_from)
        .ok_or_else(|| Error::Unknown("seq field is not found".to_owned()))?
        .map_err(|_| Error::Unknown("seq field can't be converted to EventStreamSeq".to_owned()))?;
    Ok((
        event_stream_id.to_owned(),
        event_stream_seq,
        document.update_time.unwrap(),
    ))
}
