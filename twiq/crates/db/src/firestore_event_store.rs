use std::{collections::HashMap, str::FromStr};

use google_cloud_auth::Credential;
use reqwest::Response;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::firestore_rest::{
    self, BeginTransactionRequestBody, BeginTransactionResponse, CollectionSelector,
    CommitRequestBody, Direction, Document, FieldFilter, FieldOperator, FieldReference,
    FieldTransform, Filter, Order, Precondition, Projection, RunQueryRequestBody, ServerValue,
    StructuredQuery, Timestamp, TransactionOptions, Value, Write,
};
use event_store_core::{
    event::Event, event_data::EventData, event_id::EventId, event_stream_id::EventStreamId,
    event_stream_seq::EventStreamSeq,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("status code error : {0}, {1}")]
    StatusCode(String, u16),
    #[error("unknown error : {0}")]
    Unknown(String),
}

fn event_stream_to_fields(
    event_stream_id: EventStreamId,
    event_stream_seq: EventStreamSeq,
) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    map.insert("id".to_owned(), Value::String(event_stream_id.to_string()));
    map.insert(
        "seq".to_owned(),
        Value::Integer(i64::from(event_stream_seq)),
    );
    map
}

fn event_to_fields(event: &Event) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    map.insert("id".to_owned(), Value::String(event.id().to_string()));
    map.insert(
        "stream_id".to_owned(),
        Value::String(event.stream_id().to_string()),
    );
    map.insert(
        "stream_seq".to_owned(),
        Value::Integer(i64::from(event.stream_seq())),
    );
    map.insert(
        "data".to_owned(),
        Value::String(event.clone().data().to_string()),
    );
    map
}

#[derive(Debug, thiserror::Error)]
enum TryFromEventError {
    #[error("invalid format {0}")]
    InvalidFormat(String),
    #[error("invalid value type {0}")]
    InvalidValueType(String),
    #[error("no field {0}")]
    NoField(String),
}

fn fields_to_event(fields: HashMap<String, Value>) -> Result<Event, TryFromEventError> {
    let field = "id";
    let id = fields
        .get(field)
        .ok_or_else(|| TryFromEventError::NoField(field.to_owned()))
        .and_then(|v| {
            if let Value::String(s) = v {
                Ok(s)
            } else {
                Err(TryFromEventError::InvalidValueType(field.to_owned()))
            }
        })
        .and_then(|s| {
            EventId::from_str(s).map_err(|e| TryFromEventError::InvalidFormat(e.to_string()))
        })?;
    let field = "stream_id";
    let stream_id = fields
        .get(field)
        .ok_or_else(|| TryFromEventError::NoField(field.to_owned()))
        .and_then(|v| {
            if let Value::String(s) = v {
                Ok(s)
            } else {
                Err(TryFromEventError::InvalidValueType(field.to_owned()))
            }
        })
        .and_then(|s| {
            EventStreamId::from_str(s).map_err(|e| TryFromEventError::InvalidFormat(e.to_string()))
        })?;
    let field = "stream_seq";
    let stream_seq = fields
        .get(field)
        .ok_or_else(|| TryFromEventError::NoField(field.to_owned()))
        .and_then(|v| {
            if let Value::Integer(s) = v {
                Ok(s)
            } else {
                Err(TryFromEventError::InvalidValueType(field.to_owned()))
            }
        })
        .and_then(|n| {
            EventStreamSeq::try_from(*n)
                .map_err(|e| TryFromEventError::InvalidFormat(e.to_string()))
        })?;
    let field = "data";
    let data = fields
        .get(field)
        .ok_or_else(|| TryFromEventError::NoField(field.to_owned()))
        .and_then(|v| {
            if let Value::String(s) = v {
                Ok(s)
            } else {
                Err(TryFromEventError::InvalidValueType(field.to_owned()))
            }
        })
        .and_then(|s| {
            EventData::try_from(s.to_owned())
                .map_err(|e| TryFromEventError::InvalidFormat(e.to_string()))
        })?;
    Ok(Event::new(id, stream_id, stream_seq, data))
}

pub async fn find_events_by_event_id_after(
    project_id: &str,
    credential: &Credential,
    event_id: EventId,
) -> Result<Vec<Event>, Error> {
    // TODO: begin transaction
    let database_id = "(default)";
    let parent = format!(
        "projects/{}/databases/{}/documents",
        project_id, database_id
    );

    let now = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|e| Error::Unknown(e.to_string()))?;

    let database_id = "(default)";
    let collection_id = "events";
    let document_id = event_id.to_string();
    let document_path = format!("{}/{}", collection_id, document_id);
    let name = format!(
        "projects/{}/databases/{}/documents/{}",
        project_id, database_id, document_path
    );
    let response = firestore_rest::get(credential, &name, None, None, None)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    check_status_code(&response)?;
    let document: Document = response
        .json()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    let requested_at = if let Some(Value::Timestamp(r)) = document.fields.get("requested_at") {
        Ok(Value::Timestamp(r.clone()))
    } else {
        Err(Error::Unknown(
            "requested_at is none or not timestamp".to_owned(),
        ))
    }?;

    let response = firestore_rest::run_query(
        credential,
        &parent,
        RunQueryRequestBody {
            structured_query: StructuredQuery {
                select: Projection {
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
                },
                from: vec![CollectionSelector {
                    collection_id: "events".to_owned(),
                    all_descendants: false,
                }],
                r#where: Filter::Field(FieldFilter {
                    field: FieldReference {
                        field_path: "requested_at".to_owned(),
                    },
                    op: FieldOperator::GreaterThanOrEqual,
                    value: requested_at,
                }),
                order_by: vec![Order {
                    field: FieldReference {
                        field_path: "requested_at".to_owned(),
                    },
                    direction: Direction::Ascending,
                }],
                start_at: None,
                end_at: None,
                offset: 0,
                limit: 100,
            },
            transaction: None,
            new_transaction: Some(TransactionOptions::ReadOnly { read_time: now }),
            read_time: None,
        },
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;
    check_status_code(&response)?;

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct X {
        transaction: Option<String>,
        document: Option<Document>,
        read_time: Option<Timestamp>,
        skipped_results: Option<i32>,
        done: Option<bool>,
    }
    let response: Vec<X> = response
        .json()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    let mut events = vec![];
    for r in response {
        if r.transaction.is_some() {
            continue;
        }
        if r.read_time.is_some() && r.document.is_none() {
            continue;
        }
        let document = r
            .document
            .ok_or_else(|| Error::Unknown("document is not found".to_owned()))?;
        events.push(fields_to_event(document.fields).map_err(|e| Error::Unknown(e.to_string()))?);
    }
    Ok(events)
}

pub async fn find_events_by_event_stream_id(
    project_id: &str,
    credential: &Credential,
    event_stream_id: EventStreamId,
) -> Result<Vec<Event>, Error> {
    let database_id = "(default)";
    let parent = format!(
        "projects/{}/databases/{}/documents",
        project_id, database_id
    );

    let now = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|e| Error::Unknown(e.to_string()))?;
    let response = firestore_rest::run_query(
        credential,
        &parent,
        RunQueryRequestBody {
            structured_query: StructuredQuery {
                select: Projection {
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
                },
                from: vec![CollectionSelector {
                    collection_id: "events".to_owned(),
                    all_descendants: false,
                }],
                r#where: Filter::Field(FieldFilter {
                    field: FieldReference {
                        field_path: "stream_id".to_owned(),
                    },
                    op: FieldOperator::Equal,
                    value: Value::String(event_stream_id.to_string()),
                }),
                order_by: vec![Order {
                    field: FieldReference {
                        field_path: "stream_seq".to_owned(),
                    },
                    direction: Direction::Ascending,
                }],
                start_at: None,
                end_at: None,
                offset: 0,
                limit: 100,
            },
            transaction: None,
            new_transaction: Some(TransactionOptions::ReadOnly { read_time: now }),
            read_time: None,
        },
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;
    check_status_code(&response)?;

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct X {
        transaction: Option<String>,
        document: Option<Document>,
        read_time: Option<Timestamp>,
        skipped_results: Option<i32>,
        done: Option<bool>,
    }
    let response: Vec<X> = response
        .json()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    let mut events = vec![];
    for r in response {
        if r.transaction.is_some() {
            continue;
        }
        if r.read_time.is_some() && r.document.is_none() {
            continue;
        }
        let document = r
            .document
            .ok_or_else(|| Error::Unknown("document is not found".to_owned()))?;
        events.push(fields_to_event(document.fields).map_err(|e| Error::Unknown(e.to_string()))?);
    }
    Ok(events)
}

pub async fn store(
    project_id: &str,
    credential: &Credential,
    current: Option<EventStreamSeq>,
    events: Vec<Event>,
) -> Result<(), Error> {
    if events.is_empty() {
        return Ok(());
    }

    let event_stream_id = events[0].stream_id();
    if !events
        .iter()
        .all(|event| event.stream_id() == event_stream_id)
    {
        return Err(Error::Unknown(
            "the events contains multiple event_stream_ids".to_owned(),
        ));
    }

    let last_event_stream_seq = if let Some(seq) = events.iter().try_fold(0_u32, |seq, event| {
        let next = u32::from(event.stream_seq());
        if seq < next {
            Some(next)
        } else {
            None
        }
    }) {
        EventStreamSeq::from(seq)
    } else {
        return Err(Error::Unknown(
            "the events are not in correct order".to_owned(),
        ));
    };

    let database_id = "(default)";
    let database = format!("projects/{}/databases/{}", project_id, database_id);

    let response = firestore_rest::begin_transaction(
        credential,
        &database,
        BeginTransactionRequestBody {
            options: TransactionOptions::ReadWrite {
                retry_transaction: None,
            },
        },
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;
    check_status_code(&response)?;

    let response: BeginTransactionResponse = response
        .json()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    let transaction = response.transaction;

    let mut writes = vec![];
    let collection_id = "event_streams";
    let document_id = event_stream_id.to_string();
    let event_stream_document = Document {
        name: format!(
            "projects/{}/databases/{}/documents/{}/{}",
            project_id, &database_id, collection_id, document_id
        ),
        fields: event_stream_to_fields(event_stream_id, last_event_stream_seq),
        create_time: None,
        update_time: None,
    };
    let precondition = match current {
        Some(expected_event_stream_seq) => {
            let (_, event_stream_seq, update_time) = get_event_stream(
                credential,
                project_id,
                &transaction,
                database_id,
                event_stream_id,
            )
            .await?;
            if event_stream_seq != expected_event_stream_seq {
                return Err(Error::Unknown("conflict".to_string()));
            }
            Precondition::UpdateTime(update_time)
        }
        None => Precondition::Exists(false),
    };
    writes.push(Write::Update {
        current_document: Some(precondition),
        update: event_stream_document,
        update_mask: None,
        update_transforms: None,
    });

    for event in events {
        let collection_id = "events";
        let document_id = event.id().to_string();
        writes.push(Write::Update {
            current_document: Some(Precondition::Exists(false)),
            update: Document {
                name: format!(
                    "projects/{}/databases/{}/documents/{}/{}",
                    &project_id, &database_id, collection_id, document_id
                ),
                fields: event_to_fields(&event),
                create_time: None,
                update_time: None,
            },
            update_mask: None,
            update_transforms: Some(vec![FieldTransform {
                field_path: "requested_at".to_owned(),
                set_to_server_value: Some(ServerValue::RequestTime),
            }]),
        });
    }

    firestore_rest::commit(
        credential,
        &database,
        CommitRequestBody {
            writes,
            transaction: Some(transaction),
        },
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;

    // TODO: rollback

    Ok(())
}

async fn get_event_stream(
    credential: &Credential,
    project_id: &str,
    transaction: &str,
    database_id: &str,
    event_stream_id: EventStreamId,
) -> Result<(EventStreamId, EventStreamSeq, Timestamp), Error> {
    let collection_id = "event_streams";
    let document_id = event_stream_id.to_string();
    let name = format!(
        "projects/{}/databases/{}/documents/{}/{}",
        project_id, database_id, collection_id, document_id
    );
    let response = firestore_rest::get(credential, &name, None, Some(transaction), None)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    check_status_code(&response)?;

    let document: Document = response
        .json()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    let event_stream_id = match document.fields.get("id") {
        Some(value) => {
            if let Value::String(value) = value {
                EventStreamId::from_str(value).map_err(|e| Error::Unknown(e.to_string()))?
            } else {
                return Err(Error::Unknown(
                    "get_event_stream failed: id field vaue is invalid".to_string(),
                ));
            }
        }
        None => {
            return Err(Error::Unknown(
                "get_event_stream failed: id field is not found".to_string(),
            ))
        }
    };
    let event_stream_seq = match document.fields.get("seq") {
        Some(value) => {
            if let Value::Integer(value) = value {
                EventStreamSeq::try_from(*value).map_err(|e| Error::Unknown(e.to_string()))?
            } else {
                return Err(Error::Unknown(
                    "get_event_stream failed: seq field vaue is invalid".to_string(),
                ));
            }
        }
        None => {
            return Err(Error::Unknown(
                "get_event_stream failed: seq field is not found".to_string(),
            ))
        }
    };

    Ok((
        event_stream_id,
        event_stream_seq,
        document.update_time.unwrap(),
    ))
}

fn check_status_code(response: &Response) -> Result<(), Error> {
    let status_code = response.status();
    if status_code.is_success() {
        Ok(())
    } else {
        Err(Error::StatusCode(
            response.url().to_string(),
            u16::from(status_code),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::{env, time::Duration};

    use anyhow::Context;
    use event_store_core::{
        event_data::EventData, event_id::EventId, event_stream_id::EventStreamId,
    };
    use google_cloud_auth::CredentialConfig;
    use tokio::time::sleep;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test() -> anyhow::Result<()> {
        let project_id = env::var("PROJECT_ID").context("PROJECT_ID")?;
        // GOOGLE_APPLICATION_CREDENTIALS environment variable
        let config = CredentialConfig::builder()
            .scopes(vec!["https://www.googleapis.com/auth/cloud-platform".into()])
            .build()?;
        let credential = Credential::find_default(config).await?;

        let id = EventId::generate();
        let stream_id = EventStreamId::generate();
        let stream_seq = EventStreamSeq::from(1_u32);
        let data = EventData::try_from("{}".to_owned())?;
        let event1 = Event::new(id, stream_id, stream_seq, data);
        store(&project_id, &credential, None, vec![event1.clone()]).await?;

        let stream_seq2 = stream_seq.next()?;
        let id = EventId::generate();
        let data = EventData::try_from(r#"{"foo":"bar"}"#.to_owned())?;
        let event2 = Event::new(id, stream_id, stream_seq2, data);
        store(
            &project_id,
            &credential,
            Some(stream_seq),
            vec![event2.clone()],
        )
        .await?;

        sleep(Duration::from_secs(1)).await;

        let events = find_events_by_event_stream_id(&project_id, &credential, stream_id).await?;
        assert_eq!(events, vec![event1.clone(), event2.clone()]);

        let id = EventId::generate();
        let stream_id = EventStreamId::generate();
        let stream_seq = EventStreamSeq::from(1_u32);
        let data = EventData::try_from("{}".to_owned())?;
        let event3 = Event::new(id, stream_id, stream_seq, data);
        let stream_seq2 = stream_seq.next()?;
        let id = EventId::generate();
        let data = EventData::try_from(r#"{"foo":"bar"}"#.to_owned())?;
        let event4 = Event::new(id, stream_id, stream_seq2, data);
        store(
            &project_id,
            &credential,
            None,
            vec![event3.clone(), event4.clone()],
        )
        .await?;

        sleep(Duration::from_secs(1)).await;

        let events = find_events_by_event_id_after(&project_id, &credential, event1.id()).await?;
        assert_eq!(
            events,
            vec![event1, event2.clone(), event3.clone(), event4.clone()]
        );
        let events = find_events_by_event_id_after(&project_id, &credential, event2.id()).await?;
        assert_eq!(events, vec![event2, event3, event4]);
        Ok(())
    }
}
