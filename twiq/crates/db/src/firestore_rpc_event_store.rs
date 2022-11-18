use std::{collections::HashMap, str::FromStr};

use async_trait::async_trait;
use event_store_core::{
    event_store::{self, EventStore},
    Event, EventAt, EventId, EventPayload, EventStream, EventStreamId, EventStreamSeq, EventType,
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
            precondition::ConditionType,
            run_query_request::{self},
            structured_query::{
                field_filter, filter::FilterType, CollectionSelector, Direction, FieldFilter,
                FieldReference, Filter, Order, Projection,
            },
            transaction_options::{Mode, ReadOnly},
            write::Operation,
            Document, Precondition, RunQueryRequest, StructuredQuery, TransactionOptions, Value,
            Write,
        },
        helper::{
            get_field_as_i64, get_field_as_str, get_field_as_timestamp, value_from_i64,
            value_from_string, value_from_timestamp,
        },
    },
    firestore_transaction::FirestoreTransaction,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("event_store_core::event_at {0}")]
    EventAt(#[from] event_store_core::event_at::Error),
    #[error("event_store_core::event_id {0}")]
    EventId(#[from] event_store_core::event_id::Error),
    #[error("event_store_core::event_payload {0}")]
    EventPayload(#[from] event_store_core::event_payload::Error),
    #[error("event_store_core::event_stream {0}")]
    EventStream(#[from] event_store_core::event_stream::Error),
    #[error("event_store_core::event_stream_id {0}")]
    EventStreamId(#[from] event_store_core::event_stream_id::Error),
    #[error("event_store_core::event_stream_seq {0}")]
    EventStreamSeq(#[from] event_store_core::event_stream_seq::Error),
    #[error("event_store_core::event_type {0}")]
    EventType(#[from] event_store_core::event_type::Error),
    #[error("firestore_rpc::helper {0}")]
    FirestoreRpcHelper(#[from] crate::firestore_rpc::helper::Error),
    #[error("firestore_rpc::helper::GetFieldError {0}")]
    FirestoreRpcHelperGetField(#[from] crate::firestore_rpc::helper::GetFieldError),
    #[error("google_cloud_auth {0}")]
    GoogleCloudAuth(#[from] google_cloud_auth::Error),
    #[error("prost_types::TimestampError {0}")]
    ProstTypesTimestamp(#[from] prost_types::TimestampError),
    #[error("time::error::Format {0}")]
    TimeFormat(#[from] time::error::Format),
    #[error("tonic::Status {0}")]
    TonicStatus(#[from] tonic::Status),
    #[error("unknown {0}")]
    Unknown(String),
}

impl From<Error> for event_store::Error {
    fn from(e: Error) -> Self {
        event_store::Error::Unknown(e.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct FirestoreRpcEventStore {
    transaction: FirestoreTransaction,
}

impl FirestoreRpcEventStore {
    pub fn new(transaction: FirestoreTransaction) -> Self {
        Self { transaction }
    }

    async fn client(
        &self,
    ) -> Result<
        FirestoreClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    > {
        Ok(self.transaction.client().await?)
    }

    async fn find_event(&self, event_id: EventId) -> Result<Option<Event>> {
        let document = self
            .transaction
            .get_document("events", &event_id.to_string())
            .await?;
        document.as_ref().map(event_from_fields).transpose()
    }

    async fn find_event_stream(
        &self,
        event_stream_id: EventStreamId,
    ) -> Result<Option<EventStream>> {
        let parent = self.transaction.documents_path();

        let s = OffsetDateTime::now_utc().format(&Rfc3339)?;
        let now = Timestamp::from_str(s.as_str())?;
        let mut client = self.client().await?;
        let response = client
            .run_query(RunQueryRequest {
                parent,
                query_type: Some(run_query_request::QueryType::StructuredQuery(
                    StructuredQuery {
                        select: Some(event_fields_projection()),
                        from: vec![CollectionSelector {
                            collection_id: "events".to_owned(),
                            all_descendants: false,
                        }],
                        r#where: Some(Filter {
                            filter_type: Some(FilterType::FieldFilter(FieldFilter {
                                field: Some(FieldReference {
                                    field_path: "stream_id".to_owned(),
                                }),
                                op: field_filter::Operator::Equal as i32,
                                value: Some(value_from_string(event_stream_id.to_string())),
                            })),
                        }),
                        order_by: vec![Order {
                            field: Some(FieldReference {
                                field_path: "stream_seq".to_owned(),
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
            })
            .await?;
        let mut run_query_response = response.into_inner();
        let mut events = vec![];
        while let Some(r) = run_query_response.message().await? {
            if !r.transaction.is_empty() {
                continue;
            }
            if r.read_time.is_some() && r.document.is_none() {
                continue;
            }
            let document = r
                .document
                .ok_or_else(|| Error::Unknown("document is not found".to_owned()))?;
            events.push(event_from_fields(&document)?);
        }
        Ok(if events.is_empty() {
            None
        } else {
            Some(EventStream::new(events)?)
        })
    }

    async fn find_events(&self, after: Option<EventId>) -> Result<Vec<Event>> {
        // get requested_at
        let requested_at = match after {
            Some(event_id) => self
                .transaction
                .get_document("events", &event_id.to_string())
                .await?
                .map(|document| get_field_as_timestamp(&document, "requested_at"))
                .transpose()?,
            None => None,
        };

        // get events (run_query)
        let parent = self.transaction.documents_path();
        let s = OffsetDateTime::now_utc().format(&Rfc3339)?;
        let now = Timestamp::from_str(s.as_str())?;
        let mut client = self.client().await?;
        let response = client
                .run_query(RunQueryRequest {
                    parent,
                    query_type: Some(run_query_request::QueryType::StructuredQuery(
                        StructuredQuery {
                            select: Some(event_fields_projection()),
                            from: vec![CollectionSelector {
                                collection_id: "events".to_owned(),
                                all_descendants: false,
                            }],
                            r#where: requested_at.map(|requested_at| {
                                Filter {
                                    filter_type: Some(FilterType::FieldFilter(FieldFilter {
                                        field: Some(FieldReference {
                                            field_path: "requested_at".to_owned(),
                                        }),
                                        op: field_filter::Operator::GreaterThanOrEqual as i32,
                                        value: Some(value_from_timestamp(requested_at)),
                                    }))
                                }
                            }),
                            order_by: vec![
                                Order {
                                    field: Some(FieldReference {
                                        field_path: "requested_at".to_owned(),
                                    }),
                                    direction: Direction::Ascending as i32,
                                },
                                Order {
                                    field: Some(FieldReference {
                                        field_path: "stream_id".to_owned(),
                                    }),
                                    direction: Direction::Ascending as i32,
                                },
                                Order {
                                    field: Some(FieldReference {
                                        field_path: "stream_seq".to_owned(),
                                    }),
                                    direction: Direction::Ascending as i32,
                                }
                            ],
                            start_at: None,
                            end_at: None,
                            offset: 0,
                            limit: Some(100),
                        },
                    )),
                    consistency_selector: Some(run_query_request::ConsistencySelector::NewTransaction(TransactionOptions { mode: Some(Mode::ReadOnly(ReadOnly { consistency_selector: Some(crate::firestore_rpc::google::firestore::v1::transaction_options::read_only::ConsistencySelector::ReadTime(now))})) }))
                }).await?;
        let mut run_query_response = response.into_inner();
        let mut events = vec![];
        while let Some(r) = run_query_response.message().await? {
            if !r.transaction.is_empty() {
                continue;
            }
            if r.read_time.is_some() && r.document.is_none() {
                continue;
            }
            let document = r
                .document
                .ok_or_else(|| Error::Unknown("document is not found".to_owned()))?;
            events.push(event_from_fields(&document)?);
        }
        Ok(events)
    }

    async fn store(
        &self,
        current: Option<EventStreamSeq>,
        event_stream: EventStream,
    ) -> Result<()> {
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
                // get event_stream
                let document = self
                    .transaction
                    .get_document("event_streams", &event_stream.id().to_string())
                    .await?
                    .ok_or_else(|| Error::Unknown("not found".to_owned()))?;
                let field = get_field_as_i64(&document, "seq")?;
                let event_stream_seq = EventStreamSeq::try_from(field)?;
                let update_time = document.update_time.expect("output contains update_time");

                if event_stream_seq != expected_event_stream_seq {
                    return Err(Error::Unknown("conflict".to_owned()));
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
            .await?;

        for event in event_stream.events() {
            if let Some(c) = current {
                if event.stream_seq() <= c {
                    continue;
                }
            }
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
                .await?;
        }
        Ok(())
    }
}

fn event_fields_projection() -> Projection {
    Projection {
        fields: vec![
            FieldReference {
                field_path: "id".to_owned(),
            },
            FieldReference {
                field_path: "type".to_owned(),
            },
            FieldReference {
                field_path: "stream_id".to_owned(),
            },
            FieldReference {
                field_path: "stream_seq".to_owned(),
            },
            FieldReference {
                field_path: "at".to_owned(),
            },
            FieldReference {
                field_path: "payload".to_owned(),
            },
        ],
    }
}

#[async_trait]
impl EventStore for FirestoreRpcEventStore {
    async fn find_event(&self, event_id: EventId) -> event_store::Result<Option<Event>> {
        Ok(self.find_event(event_id).await?)
    }

    async fn find_event_ids(&self, after: Option<EventId>) -> event_store::Result<Vec<EventId>> {
        Ok(self
            .find_events(after)
            .await?
            .into_iter()
            .map(|event| event.id())
            .collect::<Vec<EventId>>())
    }

    async fn find_event_stream(
        &self,
        event_stream_id: EventStreamId,
    ) -> event_store::Result<Option<EventStream>> {
        Ok(self.find_event_stream(event_stream_id).await?)
    }

    async fn find_events(&self, after: Option<EventId>) -> event_store::Result<Vec<Event>> {
        Ok(self.find_events(after).await?)
    }

    async fn store(
        &self,
        current: Option<EventStreamSeq>,
        event_stream: EventStream,
    ) -> event_store::Result<()> {
        Ok(self.store(current, event_stream).await?)
    }
}

fn event_stream_to_fields(
    event_stream_id: EventStreamId,
    event_stream_seq: EventStreamSeq,
) -> HashMap<String, Value> {
    let mut fields = HashMap::new();
    fields.insert(
        "id".to_owned(),
        value_from_string(event_stream_id.to_string()),
    );
    fields.insert(
        "seq".to_owned(),
        value_from_i64(i64::from(event_stream_seq)),
    );
    fields
}

fn event_from_fields(document: &Document) -> Result<Event> {
    let id = EventId::from_str(get_field_as_str(document, "id")?)?;
    let r#type = EventType::from_str(get_field_as_str(document, "type")?)?;
    let stream_id = EventStreamId::from_str(get_field_as_str(document, "stream_id")?)?;
    let stream_seq = EventStreamSeq::try_from(get_field_as_i64(document, "stream_seq")?)?;
    let at = EventAt::from_str(get_field_as_str(document, "at")?)?;
    let payload = EventPayload::from_str(get_field_as_str(document, "payload")?)?;
    Ok(Event::new(id, r#type, stream_id, stream_seq, at, payload))
}

fn event_to_fields(event: &Event) -> HashMap<String, Value> {
    let mut fields = HashMap::new();
    fields.insert("id".to_owned(), value_from_string(event.id().to_string()));
    fields.insert(
        "type".to_owned(),
        value_from_string(event.r#type().to_string()),
    );
    fields.insert(
        "stream_id".to_owned(),
        value_from_string(event.stream_id().to_string()),
    );
    fields.insert(
        "stream_seq".to_owned(),
        value_from_i64(i64::from(event.stream_seq())),
    );
    fields.insert("at".to_owned(), value_from_string(event.at().to_string()));
    fields.insert(
        "payload".to_owned(),
        value_from_string(event.payload().to_string()),
    );
    fields
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use event_store_core::{
        event_id::EventId, event_payload::EventPayload, event_stream_id::EventStreamId,
    };
    use tokio::time::sleep;

    use crate::config::Config;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test() -> anyhow::Result<()> {
        let config = Config::load_from_env();
        let (project_id, database_id) = (
            config.project_id().to_owned(),
            config.database_id().to_owned(),
        );
        let transaction =
            FirestoreTransaction::begin(project_id.clone(), database_id.clone()).await?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());

        let id = EventId::generate();
        let r#type = EventType::from_str("created")?;
        let stream_id = EventStreamId::generate();
        let stream_seq = EventStreamSeq::from(1_u32);
        let at = EventAt::now();
        let data = EventPayload::try_from("{}".to_owned())?;
        let event1 = Event::new(id, r#type, stream_id, stream_seq, at, data);
        let mut event_stream = EventStream::new(vec![event1.clone()])?;
        event_store.store(None, event_stream.clone()).await?;
        transaction.commit().await?;

        let transaction =
            FirestoreTransaction::begin(project_id.clone(), database_id.clone()).await?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());
        let stream_seq2 = stream_seq.next()?;
        let id = EventId::generate();
        let r#type = EventType::from_str("updated")?;
        let at = EventAt::now();
        let data = EventPayload::try_from(r#"{"foo":"bar"}"#.to_owned())?;
        let event2 = Event::new(id, r#type, stream_id, stream_seq2, at, data);
        event_stream.push_event(event2.clone())?;
        event_store
            .store(Some(stream_seq), event_stream.clone())
            .await?;
        transaction.commit().await?;

        sleep(Duration::from_secs(1)).await;

        {
            let transaction =
                FirestoreTransaction::begin(project_id.clone(), database_id.clone()).await?;
            let event_store = FirestoreRpcEventStore::new(transaction.clone());
            assert_eq!(
                event_store.find_event_stream(stream_id).await?,
                Some(event_stream)
            );
        }

        let transaction =
            FirestoreTransaction::begin(project_id.clone(), database_id.clone()).await?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());
        let id = EventId::generate();
        let r#type = EventType::from_str("created")?;
        let stream_id = EventStreamId::generate();
        let stream_seq = EventStreamSeq::from(1_u32);
        let at = EventAt::now();
        let data = EventPayload::try_from("{}".to_owned())?;
        let event3 = Event::new(id, r#type, stream_id, stream_seq, at, data);
        let stream_seq2 = stream_seq.next()?;
        let id = EventId::generate();
        let r#type = EventType::from_str("updated")?;
        let at = EventAt::now();
        let data = EventPayload::try_from(r#"{"foo":"bar"}"#.to_owned())?;
        let event4 = Event::new(id, r#type, stream_id, stream_seq2, at, data);
        let event_stream = EventStream::new(vec![event3.clone(), event4.clone()])?;
        event_store.store(None, event_stream.clone()).await?;
        transaction.commit().await?;

        sleep(Duration::from_secs(1)).await;

        let transaction =
            FirestoreTransaction::begin(project_id.clone(), database_id.clone()).await?;
        let event_store = FirestoreRpcEventStore::new(transaction.clone());
        let events = event_store.find_events(Some(event1.id())).await?;
        assert_eq!(
            events,
            vec![event1, event2.clone(), event3.clone(), event4.clone()]
        );
        let events = event_store.find_events(Some(event2.id())).await?;
        assert_eq!(events, vec![event2, event3, event4]);
        Ok(())
    }
}
