use std::{collections::HashMap, str::FromStr};

use async_trait::async_trait;
use event_store_core::{
    event_store::{self, EventStore},
    Event, EventId, EventPayload, EventStream, EventStreamId, EventStreamSeq, EventType,
};
use prost_types::Timestamp;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tonic::{codegen::InterceptedService, transport::Channel, Request, Status};

use crate::{
    firestore_rpc::{
        google::firestore::v1::{
            document_transform::{
                field_transform::{ServerValue, TransformType},
                FieldTransform,
            },
            firestore_client::FirestoreClient,
            get_document_request,
            precondition::ConditionType,
            run_query_request::{self},
            structured_query::{
                field_filter, filter::FilterType, CollectionSelector, Direction, FieldFilter,
                FieldReference, Filter, Order, Projection,
            },
            transaction_options::{Mode, ReadOnly},
            write::Operation,
            Document, GetDocumentRequest, Precondition, RunQueryRequest, StructuredQuery,
            TransactionOptions, Value, Write,
        },
        helper::{
            get_field_as_i64, get_field_as_str, get_field_as_timestamp, path::document_path,
            value_from_i64, value_from_string, value_from_timestamp,
        },
    },
    firestore_transaction::FirestoreTransaction,
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
    transaction: FirestoreTransaction,
}

impl FirestoreRpcEventStore {
    pub fn new(transaction: FirestoreTransaction) -> Self {
        Self { transaction }
    }
}

#[async_trait]
impl EventStore for FirestoreRpcEventStore {
    async fn find_event(&self, event_id: EventId) -> event_store::Result<Option<Event>> {
        let collection_id = "events";
        let document_id = event_id.to_string();
        let name = self.transaction.document_path(collection_id, &document_id);
        let response = self
            .transaction
            .client()
            .await
            .map_err(|e| event_store::Error::Unknown(e.to_string()))?
            .get_document(GetDocumentRequest {
                name,
                mask: None,
                consistency_selector: Some(get_document_request::ConsistencySelector::Transaction(
                    self.transaction.name(),
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

        // get requested_at
        let requested_at = {
            let collection_id = "events";
            let document_id = event_id.to_string();
            let name = self.transaction.document_path(collection_id, &document_id);
            let response = self
                .transaction
                .client()
                .await
                .map_err(|status| event_store::Error::Unknown(status.to_string()))?
                .get_document(GetDocumentRequest {
                    name,
                    mask: None,
                    consistency_selector: Some(
                        get_document_request::ConsistencySelector::Transaction(
                            self.transaction.name(),
                        ),
                    ),
                })
                .await
                .map_err(|status| event_store::Error::Unknown(status.to_string()))?;
            let document = response.into_inner();
            get_field_as_timestamp(&document, "requested_at").unwrap()
        };

        // get events (run_query)
        let parent = self.transaction.documents_path();
        let now = OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .map_err(|e| event_store::Error::Unknown(e.to_string()))
            .and_then(|s| {
                Timestamp::from_str(s.as_str())
                    .map_err(|e| event_store::Error::Unknown(e.to_string()))
            })?;
        let response = self
                .transaction
                .client()
                .await
                .map_err(|status| event_store::Error::Unknown(status.to_string()))?
        .run_query(RunQueryRequest {
                    parent,
                    query_type: Some(run_query_request::QueryType::StructuredQuery(
                        StructuredQuery {
                            select: Some(Projection {
                                fields: vec![
                                    FieldReference {
                                        field_path: "id".to_owned(),
                                    },
                                    FieldReference {
                                        field_path: "stream_id".to_owned(),
                                    },
                                    FieldReference {
                                        field_path: "stream_seq".to_owned(),
                                    },
                                    FieldReference {
                                        field_path: "data".to_owned(),
                                    },
                                ],
                            }),
                            from: vec![CollectionSelector {
                                collection_id: "events".to_owned(),
                                all_descendants: false,
                            }],
                            r#where: Some(Filter {
                                filter_type: Some(FilterType::FieldFilter(FieldFilter {
                                    field: Some(FieldReference {
                                        field_path: "requested_at".to_owned(),
                                    }),
                                    op: field_filter::Operator::GreaterThanOrEqual as i32,
                                    value: Some(value_from_timestamp(requested_at)),
                                })),
                            }),
                            order_by: vec![Order {
                                field: Some(FieldReference {
                                    field_path: "requested_at".to_owned(),
                                }),
                                direction: Direction::Ascending as i32,
                            }],
                            start_at: None,
                            end_at: None,
                            offset: 0,
                            limit: Some(100),
                        },
                    )),
                    consistency_selector: Some(run_query_request::ConsistencySelector::NewTransaction(TransactionOptions { mode: Some(Mode::ReadOnly(ReadOnly { consistency_selector: Some(crate::firestore_rpc::google::firestore::v1::transaction_options::read_only::ConsistencySelector::ReadTime(now))})) }))
                }).await
                        .map_err(|status| event_store::Error::Unknown(status.to_string()))?;
        let mut run_query_response = response.into_inner();
        let mut events = vec![];
        while let Some(r) = run_query_response
            .message()
            .await
            .map_err(|status| event_store::Error::Unknown(status.to_string()))?
        {
            if !r.transaction.is_empty() {
                continue;
            }
            if r.read_time.is_some() && r.document.is_none() {
                continue;
            }
            let document = r
                .document
                .ok_or_else(|| event_store::Error::Unknown("document is not found".to_owned()))?;
            events.push(
                event_from_fields(&document)
                    .map_err(|e| event_store::Error::Unknown(e.to_string()))?,
            );
        }
        Ok(events
            .into_iter()
            .map(|event| event.id())
            .collect::<Vec<EventId>>())
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
            name: self.transaction.document_path(collection_id, &document_id),
            fields: event_stream_to_fields(event_stream.id(), event_stream.seq()),
            create_time: None,
            update_time: None,
        };
        let precondition = match current {
            Some(expected_event_stream_seq) => {
                let (_, event_stream_seq, update_time) = get_event_stream(
                    &mut client,
                    &self.transaction.project_id(),
                    self.transaction.name(),
                    &self.transaction.database_id(),
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
        self.transaction
            .push_write(Write {
                update_mask: None,
                update_transforms: vec![],
                current_document: Some(precondition),
                operation: Some(Operation::Update(event_stream_document)),
            })
            .await
            .map_err(|e| event_store::Error::Unknown(e.to_string()))?;

        for event in event_stream.events() {
            let collection_id = "events";
            let document_id = event.id().to_string();
            self.transaction
                .push_write(Write {
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
                        name: self.transaction.document_path(collection_id, &document_id),
                        fields: event_to_fields(&event),
                        create_time: None,
                        update_time: None,
                    })),
                })
                .await
                .map_err(|e| event_store::Error::Unknown(e.to_string()))?;
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
    let name = document_path(project_id, database_id, collection_id, &document_id);
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
