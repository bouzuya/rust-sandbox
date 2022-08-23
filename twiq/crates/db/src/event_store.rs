use std::{collections::HashMap, env, str::FromStr};

use reqwest::Response;

use crate::{
    event::Event,
    event_stream_id::EventStreamId,
    event_stream_seq::EventStreamSeq,
    firestore_rest::{self, Document, Timestamp, Value},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown error : {0}")]
    Unknown(String),
}

pub async fn store(current: Option<EventStreamSeq>, event: Event) -> Result<(), Error> {
    let bearer_token =
        env::var("GOOGLE_BEARER_TOKEN").map_err(|e| Error::Unknown(e.to_string()))?;
    let project_id = env::var("PROJECT_ID").map_err(|e| Error::Unknown(e.to_string()))?;
    let database_id = "(default)";
    // TODO: beginTransaction
    match current {
        Some(expected_event_stream_seq) => {
            let (_, event_stream_seq, update_time) =
                get_event_stream(&bearer_token, &project_id, database_id, event.stream_id())
                    .await?;
            if event_stream_seq != expected_event_stream_seq {
                return Err(Error::Unknown("conflict".to_string()));
            }
            update_event_stream(
                &bearer_token,
                &project_id,
                database_id,
                update_time,
                event.stream_id(),
                event.stream_seq(),
            )
            .await?;
        }
        None => {
            create_event_stream(
                &bearer_token,
                &project_id,
                database_id,
                event.stream_id(),
                event.stream_seq(),
            )
            .await?;
        }
    }

    create_event(&bearer_token, &project_id, database_id, event).await?;

    // TODO: rollback

    // TODO: commit

    Ok(())
}

async fn create_event(
    bearer_token: &str,
    project_id: &str,
    database_id: &str,
    event: Event,
) -> Result<Response, Error> {
    let collection_id = "events";
    let parent = format!(
        "projects/{}/databases/{}/documents",
        project_id, database_id,
    );
    let response = firestore_rest::create_document(
        (bearer_token, project_id),
        &parent,
        collection_id,
        Some(event.id().to_string().as_str()),
        None,
        Document {
            name: "unused".to_owned(),
            fields: {
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
            },
            create_time: "unused".to_owned(),
            update_time: "unused".to_owned(),
        },
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;
    if !response.status().is_success() {
        return Err(Error::Unknown(format!(
            "create_event failed: status code ({}) is not success",
            response.status()
        )));
    }
    Ok(response)
}

async fn create_event_stream(
    bearer_token: &str,
    project_id: &str,
    database_id: &str,
    event_stream_id: EventStreamId,
    event_stream_seq: EventStreamSeq,
) -> Result<Response, Error> {
    let collection_id = "event_streams";
    let parent = format!(
        "projects/{}/databases/{}/documents",
        &project_id, &database_id,
    );
    let response = firestore_rest::create_document(
        (bearer_token, project_id),
        &parent,
        collection_id,
        Some(event_stream_id.to_string().as_str()),
        None,
        Document {
            name: "unused".to_owned(),
            fields: {
                let mut map = HashMap::new();
                map.insert("id".to_owned(), Value::String(event_stream_id.to_string()));
                map.insert(
                    "stream_seq".to_owned(),
                    Value::Integer(i64::from(event_stream_seq)),
                );
                map
            },
            create_time: "unused".to_owned(),
            update_time: "unused".to_owned(),
        },
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;
    if !response.status().is_success() {
        // TODO: rollback
        return Err(Error::Unknown(format!(
            "create_event_stream failed: status code ({}) is not success",
            response.status()
        )));
    }
    Ok(response)
}

async fn get_event_stream(
    bearer_token: &str,
    project_id: &str,
    database_id: &str,
    event_stream_id: EventStreamId,
) -> Result<(EventStreamId, EventStreamSeq, Timestamp), Error> {
    let collection_id = "event_streams";
    let document_id = event_stream_id.to_string();
    let name = format!(
        "projects/{}/databases/{}/documents/{}/{}",
        project_id, database_id, collection_id, document_id
    );
    let response = firestore_rest::get((bearer_token, project_id), &name, None, None, None)
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
                "get_event_stream failed: id field is not found".to_string(),
            ))
        }
    };

    Ok((event_stream_id, event_stream_seq, document.update_time))
}

async fn update_event_stream(
    bearer_token: &str,
    project_id: &str,
    database_id: &str,
    current_document_update_time: Timestamp,
    event_stream_id: EventStreamId,
    event_stream_seq: EventStreamSeq,
) -> Result<Response, Error> {
    let collection_id = "event_streams";
    let document_id = event_stream_id;
    let document_name = format!(
        "projects/{}/databases/{}/documents/{}/{}",
        &project_id, &database_id, collection_id, document_id
    );
    let response = firestore_rest::patch(
        (bearer_token, project_id),
        &document_name,
        None,
        None,
        Some(true),
        Some(current_document_update_time),
        Document {
            name: document_name.clone(),
            fields: {
                let mut map = HashMap::new();
                map.insert("id".to_owned(), Value::String(event_stream_id.to_string()));
                map.insert(
                    "stream_seq".to_owned(),
                    Value::Integer(i64::from(event_stream_seq)),
                );
                map
            },
            create_time: "unused".to_owned(),
            update_time: "unused".to_owned(),
        },
    )
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;
    if !response.status().is_success() {
        return Err(Error::Unknown(format!(
            "update_event_stream failed: status code ({}) is not success",
            response.status()
        )));
    }
    Ok(response)
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
