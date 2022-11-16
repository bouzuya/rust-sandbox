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

    async fn client(
        &self,
    ) -> event_store::Result<
        FirestoreClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    > {
        self.transaction
            .client()
            .await
            .map_err(|e| event_store::Error::Unknown(e.to_string()))
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
        self.transaction
            .get_document("events", &event_id.to_string())
            .await
            .map_err(|e| event_store::Error::Unknown(e.to_string()))
            .and_then(|document| {
                document
                    .as_ref()
                    .map(event_from_fields)
                    .transpose()
                    .map_err(|e| event_store::Error::Unknown(e.to_string()))
            })
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
        let parent = self.transaction.documents_path();

        let now = OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .map_err(|e| event_store::Error::Unknown(e.to_string()))
            .and_then(|s| {
                Timestamp::from_str(s.as_str())
                    .map_err(|e| event_store::Error::Unknown(e.to_string()))
            })?;
        let response = self
            .client()
            .await?
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
            .await
                    .map_err(|e| event_store::Error::Unknown(e.to_string()))?;
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
        Ok(if events.is_empty() {
            None
        } else {
            Some(EventStream::new(events).map_err(|e| event_store::Error::Unknown(e.to_string()))?)
        })
    }

    async fn find_events(&self, after: Option<EventId>) -> event_store::Result<Vec<Event>> {
        // get requested_at
        let requested_at = match after {
            Some(event_id) => self
                .transaction
                .get_document("events", &event_id.to_string())
                .await
                .map_err(|status| event_store::Error::Unknown(status.to_string()))?
                .and_then(|document| get_field_as_timestamp(&document, "requested_at"))
                .map(Some)
                .ok_or_else(|| event_store::Error::Unknown("not found".to_owned()))?,
            None => None,
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
                .client()
                .await?
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
        Ok(events)
    }

    async fn store(
        &self,
        current: Option<EventStreamSeq>,
        event_stream: EventStream,
    ) -> event_store::Result<()> {
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
                    .await
                    .map_err(|e| event_store::Error::Unknown(e.to_string()))?
                    .ok_or_else(|| event_store::Error::Unknown("not found".to_owned()))?;
                let event_stream_seq = get_field_as_i64(&document, "seq")
                    .map(EventStreamSeq::try_from)
                    .ok_or_else(|| {
                        event_store::Error::Unknown("seq field is not found".to_owned())
                    })?
                    .map_err(|_| {
                        event_store::Error::Unknown(
                            "seq field can't be converted to EventStreamSeq".to_owned(),
                        )
                    })?;
                let update_time = document.update_time.expect("output contains update_time");

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
    let id = get_field_as_str(document, "id")
        .map(EventId::from_str)
        .transpose()
        .map_err(|_| Error::Unknown("id is not well-formed".to_owned()))?
        .ok_or_else(|| Error::Unknown("id is not found".to_owned()))?;
    let r#type = get_field_as_str(document, "type")
        .map(EventType::from_str)
        .transpose()
        .map_err(|_| Error::Unknown("type is not well-formed".to_owned()))?
        .ok_or_else(|| Error::Unknown("type is not found".to_owned()))?;
    let stream_id = get_field_as_str(document, "stream_id")
        .map(EventStreamId::from_str)
        .transpose()
        .map_err(|_| Error::Unknown("stream_id is not well-formed".to_owned()))?
        .ok_or_else(|| Error::Unknown("stream_id is not found".to_owned()))?;
    let stream_seq = get_field_as_i64(document, "stream_seq")
        .map(EventStreamSeq::try_from)
        .transpose()
        .map_err(|_| Error::Unknown("stream_id is not well-formed".to_owned()))?
        .ok_or_else(|| Error::Unknown("stream_id is not found".to_owned()))?;
    let at = get_field_as_str(document, "at")
        .map(EventAt::from_str)
        .transpose()
        .map_err(|_| Error::Unknown("at is not well-formed".to_owned()))?
        .ok_or_else(|| Error::Unknown("at is not found".to_owned()))?;
    let payload = get_field_as_str(document, "payload")
        .map(EventPayload::from_str)
        .transpose()
        .map_err(|_| Error::Unknown("payload is not well-formed".to_owned()))?
        .ok_or_else(|| Error::Unknown("payload is not found".to_owned()))?;
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
