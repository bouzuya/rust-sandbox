use std::env;

use reqwest::Response;
use serde_json::json;

async fn createDocument() -> anyhow::Result<Response> {
    // await setDoc(doc(db, "cities", "LA"), {
    //   name: "Los Angeles",
    //   state: "CA",
    //   country: "USA"
    // });
    //
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/createDocument>

    let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
    let project_id = env::var("PROJECT_ID")?;
    let database_id = "(default)";
    let collection_id = "cities";
    let document_id = "LA";
    let path = format!(
        "/projects/{}/databases/{}/documents/{}?documentId={}",
        project_id, database_id, collection_id, document_id
    );
    let url = format!("https://firestore.googleapis.com/v1{}", path);
    let body = json!({
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
    let client = reqwest::Client::new();
    Ok(client
        .post(url)
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
        let response = createDocument().await?;
        let status = response.status();
        assert_eq!(status, 200);
        // assert_eq!(response.bytes().await?, "");
        Ok(())
    }
}
