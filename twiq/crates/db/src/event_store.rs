use std::{collections::HashMap, env, str::FromStr};

use reqwest::Response;

use crate::{
    event::Event,
    event_stream_id::EventStreamId,
    event_stream_seq::EventStreamSeq,
    firestore_rest::{
        self, BeginTransactionRequestBody, BeginTransactionResponse, CommitRequestBody, Document,
        Precondition, Timestamp, TransactionOptions, Value, Write,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
    map.insert("data".to_owned(), Value::String(event.data().to_string()));
    map
}

pub async fn store(current: Option<EventStreamSeq>, event: Event) -> Result<(), Error> {
    let bearer_token =
        env::var("GOOGLE_BEARER_TOKEN").map_err(|e| Error::Unknown(e.to_string()))?;
    let project_id = env::var("PROJECT_ID").map_err(|e| Error::Unknown(e.to_string()))?;
    let database_id = "(default)";
    let database = format!("projects/{}/databases/{}", project_id, database_id);

    let response = firestore_rest::begin_transaction(
        (&bearer_token, &project_id),
        &database,
        BeginTransactionRequestBody {
            options: TransactionOptions::ReadWrite {
                retry_transaction: None,
            },
        },
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;
    if !response.status().is_success() {
        return Err(Error::Unknown(format!(
            "begin_transaction failed: status code ({}) is not success",
            response.status()
        )));
    }
    let response: BeginTransactionResponse = response
        .json()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    let transaction = response.transaction;

    let mut writes = vec![];
    match current {
        Some(expected_event_stream_seq) => {
            let (_, event_stream_seq, update_time) = get_event_stream(
                &bearer_token,
                &project_id,
                &transaction,
                database_id,
                event.stream_id(),
            )
            .await?;
            if event_stream_seq != expected_event_stream_seq {
                return Err(Error::Unknown("conflict".to_string()));
            }
            let collection_id = "event_streams";
            let document_id = event.stream_id().to_string();
            writes.push(Write::Update {
                current_document: Some(Precondition::UpdateTime(update_time)),
                update: Document {
                    name: format!(
                        "projects/{}/databases/{}/documents/{}/{}",
                        &project_id, &database_id, collection_id, document_id
                    ),
                    fields: event_stream_to_fields(event.stream_id(), event.stream_seq()),
                    create_time: None,
                    update_time: None,
                },
                update_mask: None,
            });
        }
        None => {
            let collection_id = "event_streams";
            let document_id = event.stream_id().to_string();
            writes.push(Write::Update {
                current_document: Some(Precondition::Exists(false)),
                update: Document {
                    name: format!(
                        "projects/{}/databases/{}/documents/{}/{}",
                        &project_id, &database_id, collection_id, document_id
                    ),
                    fields: event_stream_to_fields(event.stream_id(), event.stream_seq()),
                    create_time: None,
                    update_time: None,
                },
                update_mask: None,
            });
        }
    }

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
    });

    firestore_rest::commit(
        (&bearer_token, &project_id),
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
    bearer_token: &str,
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
    let response = firestore_rest::get(
        (bearer_token, project_id),
        &name,
        None,
        Some(transaction),
        None,
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;
    if !response.status().is_success() {
        return Err(Error::Unknown(format!(
            "get_event_stream failed: status code ({}) is not success",
            response.status()
        )));
    }
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

#[cfg(test)]
mod tests {
    use crate::{event_data::EventData, event_id::EventId, event_stream_id::EventStreamId};

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test() -> anyhow::Result<()> {
        let id = EventId::generate();
        let stream_id = EventStreamId::generate();
        let stream_seq = EventStreamSeq::from(1_u32);
        let data = EventData::try_from("{}".to_owned())?;
        let event = Event::new(id, stream_id, stream_seq, data);
        store(None, event).await?;

        let stream_seq2 = EventStreamSeq::from(u32::from(stream_seq) + 1);
        let id = EventId::generate();
        let data = EventData::try_from(r#"{"foo":"bar"}"#.to_owned())?;
        let event2 = Event::new(id, stream_id, stream_seq2, data);
        store(Some(stream_seq), event2).await?;
        Ok(())
    }
}