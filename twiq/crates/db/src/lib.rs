pub mod event;
pub mod event_data;
pub mod event_id;
pub mod event_stream_id;
pub mod event_stream_seq;
pub mod firestore_rest;

use std::env;

use reqwest::Response;
use serde_json::json;

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
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/createDocument>

    let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
    let project_id = env::var("PROJECT_ID")?;
    let database_id = "(default)";
    let parent = format!(
        "projects/{}/databases/{}/documents",
        project_id, database_id,
    );
    let collection_id = "cities";
    let document_id = "LA";
    let document = json!({
      "fields": {
        "name": {
          "stringValue": "Los Angeles"
        },
        "state": {
          "stringValue": "CA"
        },
        "country": {
          "stringValue": "USA"
        }
      }
    });
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
async fn patch() -> anyhow::Result<Response> {
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/patch>

    let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
    let project_id = env::var("PROJECT_ID")?;
    let database_id = "(default)";
    let collection_id = "cities";
    let document_id = "LA2";
    // TODO: updateMask.fieldPaths, mask.fieldPaths, currentDocument.exists, currentDocument.updateTime
    let path = format!(
        "/projects/{}/databases/{}/documents/{}/{}",
        project_id, database_id, collection_id, document_id
    );
    let url = format!("https://firestore.googleapis.com/v1{}", path);
    let body = json!({
      "fields": {
        "name": {
          "stringValue": "Los Angeles2"
        },
        "state": {
          "stringValue": "CA2"
        },
        "country": {
          "stringValue": "USA2"
        }
      }
    });
    let client = reqwest::Client::new();
    Ok(client
        .patch(url)
        .header("Authorization", format!("Bearer {}", bearer_token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .body(serde_json::to_string(&body)?)
        .send()
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let response = create_document_example().await?;
        // let response = patch().await?;
        // let response = get().await?;
        let status = response.status();
        assert_eq!(status, 200);
        assert_eq!(response.bytes().await?, "");
        Ok(())
    }
}
