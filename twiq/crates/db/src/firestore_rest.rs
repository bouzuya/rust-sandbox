mod array_value;
mod begin_transaction_request_body;
mod begin_transaction_response;
mod collection_selector;
mod commit_request_body;
mod composite_filter;
mod composite_operator;
mod cursor;
mod direction;
mod document;
mod document_mask;
mod field_filter;
mod field_operator;
mod field_reference;
mod field_transform;
mod filter;
mod latlng;
mod map_value;
mod order;
mod precondition;
mod projection;
mod run_query_request_body;
mod server_value;
mod structured_query;
mod timestamp;
mod transaction_options;
mod unary_filter;
mod unary_operator;
mod value;
mod write;

pub use self::array_value::ArrayValue;
pub use self::begin_transaction_request_body::BeginTransactionRequestBody;
pub use self::begin_transaction_response::BeginTransactionResponse;
pub use self::collection_selector::CollectionSelector;
pub use self::commit_request_body::CommitRequestBody;
pub use self::composite_filter::CompositeFilter;
pub use self::composite_operator::CompositeOperator;
pub use self::cursor::Cursor;
pub use self::direction::Direction;
pub use self::document::Document;
pub use self::document_mask::DocumentMask;
pub use self::field_filter::FieldFilter;
pub use self::field_operator::FieldOperator;
pub use self::field_reference::FieldReference;
pub use self::field_transform::FieldTransform;
pub use self::filter::Filter;
pub use self::latlng::LatLng;
pub use self::map_value::MapValue;
pub use self::order::Order;
pub use self::precondition::Precondition;
pub use self::projection::Projection;
pub use self::run_query_request_body::RunQueryRequestBody;
pub use self::server_value::ServerValue;
pub use self::structured_query::StructuredQuery;
pub use self::timestamp::Timestamp;
pub use self::transaction_options::TransactionOptions;
pub use self::unary_filter::UnaryFilter;
pub use self::unary_operator::UnaryOperator;
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

pub async fn run_query(
    (token, project_id): (&str, &str),
    // "projects/{project_id}/databases/{databaseId}/documents"
    // or
    // "projects/{project_id}/databases/{databaseId}/documents/{document_path}"
    parent: &str,
    body: RunQueryRequestBody,
) -> anyhow::Result<Response> {
    // <https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases.documents/runQuery>
    let method = Method::POST;
    let url = format!("https://firestore.googleapis.com/v1/{}:runQuery", parent);
    Ok(Client::new()
        .request(method, url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("X-Goog-User-Project", project_id)
        .body(serde_json::to_string(&body)?)
        .send()
        .await?)
}
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    use super::*;

    pub fn serde_test<T: std::fmt::Debug + Eq + serde::de::DeserializeOwned + serde::Serialize>(
        o: T,
        s: &str,
    ) -> anyhow::Result<()> {
        assert_eq!(serde_json::from_str::<'_, T>(s)?, o);
        assert_eq!(serde_json::to_string(&o)?, s);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn begin_transaction_test() -> anyhow::Result<()> {
        let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let database = format!("projects/{}/databases/{}", project_id, database_id);
        let response = begin_transaction(
            (&bearer_token, &project_id),
            &database,
            BeginTransactionRequestBody {
                options: TransactionOptions::ReadWrite {
                    retry_transaction: None,
                },
            },
        )
        .await?;
        assert_eq!(response.status(), 200);
        let _: BeginTransactionResponse = response.json().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn commit_test() -> anyhow::Result<()> {
        let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let database = format!("projects/{}/databases/{}", project_id, database_id);
        let response = commit(
            (&bearer_token, &project_id),
            &database,
            CommitRequestBody {
                writes: vec![Write::Update {
                    current_document: None,
                    update: Document {
                        name: format!("{}/documents/cities/LA", database),
                        fields: {
                            let mut map = HashMap::new();
                            map.insert("commit".to_owned(), Value::String("commit1".to_owned()));
                            map
                        },
                        create_time: None,
                        update_time: None,
                    },
                    update_mask: None,
                }],
                transaction: None,
            },
        )
        .await?;
        assert_eq!(response.status(), 200);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn create_document_test() -> anyhow::Result<()> {
        let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let parent = format!(
            "projects/{}/databases/{}/documents",
            project_id, database_id,
        );
        let collection_id = "cities";
        let document_id = "LA";
        let document = Document {
            name: "unused".to_string(),
            fields: {
                let mut map = HashMap::new();
                map.insert("name".to_string(), Value::String("Los Angeles".to_string()));
                map.insert("state".to_string(), Value::String("CA".to_string()));
                map.insert("country".to_string(), Value::String("USA".to_string()));
                map
            },
            create_time: None,
            update_time: None,
        };
        let response = create_document(
            (&bearer_token, &project_id),
            &parent,
            collection_id,
            Some(document_id),
            None,
            document,
        )
        .await?;
        assert_eq!(response.status(), 200);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn get_test() -> anyhow::Result<()> {
        let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let collection_id = "cities";
        let document_id = "LA";
        let document_path = format!("{}/{}", collection_id, document_id);
        let name = format!(
            "projects/{}/databases/{}/documents/{}",
            project_id, database_id, document_path
        );
        let response = get((&bearer_token, &project_id), &name, None, None, None).await?;
        assert_eq!(response.status(), 200);
        let _: Document = response.json().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn patch_test() -> anyhow::Result<()> {
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
            create_time: None,
            update_time: None,
        };
        let response = patch(
            (&bearer_token, &project_id),
            &document_name,
            None,
            None,
            None,
            None,
            document,
        )
        .await?;
        assert_eq!(response.status(), 200);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn run_query_test() -> anyhow::Result<()> {
        let event_stream_id = "f9b7139d-2310-4dee-83db-c61d81f67f10";

        let now = OffsetDateTime::now_utc().format(&Rfc3339)?;
        let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let parent = format!(
            "projects/{}/databases/{}/documents",
            project_id, database_id
        );
        let response = run_query(
            (&bearer_token, &project_id),
            &parent,
            RunQueryRequestBody {
                structured_query: StructuredQuery {
                    select: Projection {
                        fields: vec![FieldReference {
                            field_path: "id".to_owned(),
                        }],
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
                        value: Value::String(event_stream_id.to_owned()),
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
        .await?;
        assert_eq!(response.status(), 200);
        assert_eq!(response.text().await?, "");
        Ok(())
    }
}
