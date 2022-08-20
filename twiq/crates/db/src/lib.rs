pub mod event;
pub mod event_data;
pub mod event_id;
pub mod event_stream_id;
pub mod event_stream_seq;
pub mod firestore_rest;

use std::{collections::HashMap, env};

use reqwest::Response;

use crate::firestore_rest::{Document, Value};

// select (one)
async fn get() -> anyhow::Result<Response> {
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/get>

    let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
    let project_id = env::var("PROJECT_ID")?;
    let database_id = "(default)";
    let collection_id = "cities";
    let document_id = "LA";
    let document_path = format!("{}/{}", collection_id, document_id);
    // TODO: mask.fieldPaths, transaction, readTime
    let path = format!(
        "/projects/{}/databases/{}/documents/{}",
        project_id, database_id, document_path
    );
    let url = format!("https://firestore.googleapis.com/v1{}", path);
    let client = reqwest::Client::new();
    Ok(client
        .get(url)
        .header("Authorization", format!("Bearer {}", bearer_token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .send()
        .await?)
}

// insert
async fn create_document_example() -> anyhow::Result<Response> {
    let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
    let project_id = env::var("PROJECT_ID")?;
    let database_id = "(default)";
    let parent = format!(
        "projects/{}/databases/{}/documents",
        project_id, database_id,
    );
    let collection_id = "cities";
    let document_id = "LA3";
    let document = Document {
        name: "unused".to_string(),
        fields: {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Value::String("Los Angeles".to_string()));
            map.insert("state".to_string(), Value::String("CA".to_string()));
            map.insert("country".to_string(), Value::String("USA".to_string()));
            map
        },
        create_time: "unused".to_string(),
        update_time: "unused".to_string(),
    };
    firestore_rest::create_document(
        (&bearer_token, &project_id),
        &parent,
        collection_id,
        Some(document_id),
        None,
        document,
    )
    .await
}

// update or insert
async fn patch_example() -> anyhow::Result<Response> {
    let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
    let project_id = env::var("PROJECT_ID")?;
    let database_id = "(default)";
    let collection_id = "cities";
    let document_id = "LA2";
    let document_name = format!(
        "projects/{}/databases/{}/documents/{}/{}",
        project_id, database_id, collection_id, document_id
    );
    let document = Document {
        name: "unused".to_string(),
        fields: {
            let mut map = HashMap::new();
            map.insert("name".to_string(), Value::String("Los Angeles".to_string()));
            map.insert("state".to_string(), Value::String("CA".to_string()));
            map.insert("country".to_string(), Value::String("USA".to_string()));
            map
        },
        create_time: "unused".to_string(),
        update_time: "unused".to_string(),
    };
    firestore_rest::patch(
        (&bearer_token, &project_id),
        &document_name,
        None,
        None,
        None,
        None,
        document,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test() -> anyhow::Result<()> {
        // let response = create_document_example().await?;
        let response = patch_example().await?;
        // let response = get().await?;
        let status = response.status();
        assert_eq!(status, 200);
        assert_eq!(response.bytes().await?, "");
        Ok(())
    }
}
