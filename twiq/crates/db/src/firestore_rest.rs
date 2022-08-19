use reqwest::{Client, Method, Response, Url};
use serde_json::Value;

pub async fn create_document(
    (token, project_id): (&str, &str),
    parent: &str,
    collection_id: &str,
    document_id: Option<&str>,
    mask_field_paths: Option<Vec<&str>>,
    document: Value,
) -> anyhow::Result<Response> {
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/createDocument>
    let method = Method::POST;
    let url = format!(
        "https://firestore.googleapis.com/v1/{}/{}",
        parent, collection_id
    );
    let mut url = Url::parse(&url)?;
    if let Some(document_id) = document_id {
        url.query_pairs_mut().append_pair("documentId", document_id);
    }
    if let Some(mask_field_paths) = mask_field_paths {
        for mask_field_path in mask_field_paths {
            url.query_pairs_mut()
                .append_pair("mask.fieldPaths", mask_field_path);
        }
    }
    Ok(Client::new()
        .request(method, url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .body(serde_json::to_string(&document)?)
        .send()
        .await?)
}
