use reqwest::{Client, Method, Response};
use serde_json::Value;

pub async fn create_document(
    (token, project_id): (&str, &str),
    parent: &str,
    collection_id: &str,
    document_id: &str,
    document: Value,
) -> anyhow::Result<Response> {
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/createDocument>
    let method = Method::POST;
    let url = format!(
        "https://firestore.googleapis.com/v1/{}/{}?documentId={}",
        parent, collection_id, document_id
    );
    Ok(Client::new()
        .request(method, url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .body(serde_json::to_string(&document)?)
        .send()
        .await?)
}
