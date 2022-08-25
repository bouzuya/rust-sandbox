mod array_value;
mod begin_transaction_request_body;
mod begin_transaction_response;
mod commit_request_body;
mod document;
mod document_mask;
mod field_reference;
mod latlng;
mod map_value;
mod precondition;
mod timestamp;
mod transaction_options;
mod value;
mod write;

pub use self::array_value::ArrayValue;
pub use self::begin_transaction_request_body::BeginTransactionRequestBody;
pub use self::begin_transaction_response::BeginTransactionResponse;
pub use self::commit_request_body::CommitRequestBody;
pub use self::document::Document;
pub use self::document_mask::DocumentMask;
pub use self::field_reference::FieldReference;
pub use self::latlng::LatLng;
pub use self::map_value::MapValue;
pub use self::precondition::Precondition;
pub use self::timestamp::Timestamp;
pub use self::transaction_options::TransactionOptions;
pub use self::value::Value;
pub use self::write::Write;
use reqwest::{Client, Method, Response, Url};

pub async fn begin_transaction(
    (token, project_id): (&str, &str),
    database: &str,
    body: BeginTransactionRequestBody,
) -> anyhow::Result<Response> {
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/beginTransaction>
    let method = Method::POST;
    let url = format!(
        "https://firestore.googleapis.com/v1/{}/documents:beginTransaction",
        database
    );

    Ok(Client::new()
        .request(method, url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .body(serde_json::to_string(&body)?)
        .send()
        .await?)
}

pub async fn commit(
    (token, project_id): (&str, &str),
    database: &str,
    body: CommitRequestBody,
) -> anyhow::Result<Response> {
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/commit>
    let method = Method::POST;
    let url = format!(
        "https://firestore.googleapis.com/v1/{}/documents:commit",
        database
    );
    Ok(Client::new()
        .request(method, url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .body(serde_json::to_string(&body)?)
        .send()
        .await?)
}

pub async fn create_document(
    (token, project_id): (&str, &str),
    parent: &str,
    collection_id: &str,
    document_id: Option<&str>,
    mask_field_paths: Option<Vec<&str>>,
    document: Document,
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
    let mut value = serde_json::to_value(document)?;
    let mut map = serde_json::Map::new();
    map.insert("fields".to_string(), value["fields"].take());
    let body = serde_json::Value::Object(map);
    Ok(Client::new()
        .request(method, url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .body(serde_json::to_string(&body)?)
        .send()
        .await?)
}

pub async fn get(
    (token, project_id): (&str, &str),
    name: &str,
    mask_field_paths: Option<Vec<&str>>,
    transaction: Option<&str>,
    read_time: Option<&str>,
) -> anyhow::Result<Response> {
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/get>

    let method = Method::GET;
    let url = format!("https://firestore.googleapis.com/v1/{}", name);
    let mut url = Url::parse(&url)?;
    if let Some(mask_field_paths) = mask_field_paths {
        for mask_field_path in mask_field_paths {
            url.query_pairs_mut()
                .append_pair("mask.fieldPaths", mask_field_path);
        }
    }
    if let Some(transaction) = transaction {
        url.query_pairs_mut()
            .append_pair("transaction", transaction);
    }
    if let Some(read_time) = read_time {
        url.query_pairs_mut().append_pair("readTime", read_time);
    }
    let client = reqwest::Client::new();
    Ok(client
        .request(method, url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .send()
        .await?)
}

pub async fn patch(
    (token, project_id): (&str, &str),
    document_name: &str,
    update_mask_field_paths: Option<Vec<&str>>,
    mask_field_paths: Option<Vec<&str>>,
    current_document_exists: Option<bool>,
    current_document_update_time: Option<Timestamp>,
    document: Document,
) -> anyhow::Result<Response> {
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/patch>
    let method = Method::PATCH;
    let url = format!("https://firestore.googleapis.com/v1/{}", document_name);
    let mut url = Url::parse(&url)?;
    if let Some(update_mask_field_paths) = update_mask_field_paths {
        for update_mask_field_path in update_mask_field_paths {
            url.query_pairs_mut()
                .append_pair("updateMask.fieldPaths", update_mask_field_path);
        }
    }
    if let Some(mask_field_paths) = mask_field_paths {
        for mask_field_path in mask_field_paths {
            url.query_pairs_mut()
                .append_pair("mask.fieldPaths", mask_field_path);
        }
    }
    if let Some(current_document_exists) = current_document_exists {
        url.query_pairs_mut().append_pair(
            "currentDocument.exists",
            current_document_exists.to_string().as_str(),
        );
    }
    if let Some(current_document_update_time) = current_document_update_time {
        url.query_pairs_mut().append_pair(
            "currentDocument.updateTime",
            current_document_update_time.as_str(),
        );
    }
    let mut value = serde_json::to_value(document)?;
    let mut map = serde_json::Map::new();
    map.insert("name".to_string(), value["name"].take());
    map.insert("fields".to_string(), value["fields"].take());
    let body = serde_json::Value::Object(map);
    Ok(Client::new()
        .request(method, url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .body(serde_json::to_string(&body)?)
        .send()
        .await?)
}
