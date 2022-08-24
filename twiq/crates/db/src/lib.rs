pub mod event;
pub mod event_data;
pub mod event_id;
pub mod event_store;
pub mod event_stream_id;
pub mod event_stream_seq;
pub mod firestore_rest;

use std::{collections::HashMap, env};

use anyhow::ensure;
use reqwest::Response;

use crate::firestore_rest::{
    BeginTransactionRequestBody, BeginTransactionResponse, CommitRequestBody, Document,
    TransactionOptions, Value, Write,
};

async fn begin_transaction_example() -> anyhow::Result<BeginTransactionResponse> {
    let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
    let project_id = env::var("PROJECT_ID")?;
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
    .await?;
    ensure!(response.status() == 200);
    let response: BeginTransactionResponse = response.json().await?;
    Ok(response)
}

async fn commit_example() -> anyhow::Result<Response> {
    let bearer_token = env::var("GOOGLE_BEARER_TOKEN")?;
    let project_id = env::var("PROJECT_ID")?;
    let database_id = "(default)";
    let database = format!("projects/{}/databases/{}", project_id, database_id);
    let response = firestore_rest::commit(
        (&bearer_token, &project_id),
        &database,
        CommitRequestBody {
            writes: vec![Write::Update {
                current_document: None,
                update: Document {
                    name: format!("{}/cities/LA", database),
                    fields: {
                        let mut map = HashMap::new();
                        map.insert("commit".to_owned(), Value::String("commit1".to_owned()));
                        map
                    },
                    create_time: "unused".to_owned(),
                    update_time: "unused".to_owned(),
                },
                update_mask: None,
            }],
            transaction: None,
        },
    )
    .await?;
    ensure!(response.status() == 200);
    Ok(response)
}

// select (one)
async fn get_example() -> anyhow::Result<Document> {
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
    let response =
        firestore_rest::get((&bearer_token, &project_id), &name, None, None, None).await?;
    ensure!(response.status() == 200);
    let document: Document = response.json().await?;
    Ok(document)
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
        let response = begin_transaction_example().await?;
        assert_eq!(serde_json::to_string(&response)?, "");
        // let response = create_document_example().await?;
        // let status = response.status();
        // assert_eq!(status, 200);

        // let response = patch_example().await?;
        // let document = get_example().await?;
        // assert_eq!(serde_json::to_string(&document)?, "");
        Ok(())
    }
}
