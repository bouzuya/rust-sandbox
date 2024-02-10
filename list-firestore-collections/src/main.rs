use std::{collections::BTreeSet, path::Path, str::FromStr as _};

use async_recursion::async_recursion;
use clap::Parser;
use firestore_path::{CollectionId, CollectionName, DatabaseName, DocumentName};
use google_api_proto::google::firestore::v1::{
    firestore_client::FirestoreClient, DocumentMask, ListCollectionIdsRequest, ListDocumentsRequest,
};
use google_authz::{Credentials, GoogleAuthz};
use tonic::transport::Channel;

type MyFirestoreClient = FirestoreClient<GoogleAuthz<Channel>>;

#[derive(clap::Parser)]
struct Args {
    #[clap(env, long)]
    google_application_credentials: String,
    #[clap(env, long)]
    project_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut client = build_client(&args.google_application_credentials).await?;
    let database_name = DatabaseName::from_project_id(args.project_id)?;
    let document_name_string = database_name.root_document_name().to_string();
    let document_names = list_document_names_recursive(&mut client, document_name_string).await?;

    println!("{:#?}", document_names);
    Ok(())
}

#[async_recursion]
async fn list_document_names_recursive(
    client: &mut MyFirestoreClient,
    parent_document_name: String,
) -> anyhow::Result<BTreeSet<String>> {
    let mut set = BTreeSet::new();
    let collection_ids = list_collection_ids_all(client, parent_document_name.clone()).await?;
    for collection_id in &collection_ids {
        let collection_name =
            CollectionName::from_str(&format!("{}/{}", parent_document_name, collection_id))?;
        let document_names = list_document_names_all(client, &collection_name).await?;
        for document_name in document_names {
            set.extend(list_document_names_recursive(client, document_name.to_string()).await?);
        }
    }
    Ok(set)
}

async fn build_client(google_application_credentials: &str) -> anyhow::Result<MyFirestoreClient> {
    let service = Channel::from_static("https://firestore.googleapis.com")
        .connect()
        .await?;
    let client = GoogleAuthz::builder(service)
        .credentials(
            Credentials::builder()
                .json_file(Path::new(google_application_credentials))
                .build()
                .await?,
        )
        .build()
        .await;
    Ok(FirestoreClient::new(client))
}

async fn list_collection_ids_all(
    client: &mut MyFirestoreClient,
    parent: String,
) -> anyhow::Result<Vec<CollectionId>> {
    let mut collection_ids = vec![];
    let mut next_page_token = String::new();
    loop {
        let response = client
            .list_collection_ids(ListCollectionIdsRequest {
                page_token: next_page_token.clone(),
                parent: parent.clone(),
                ..Default::default()
            })
            .await?
            .into_inner();
        collection_ids.extend(response.collection_ids);
        if response.next_page_token.is_empty() {
            break;
        }
        next_page_token = response.next_page_token;
    }
    collection_ids
        .into_iter()
        .map(|s| Ok(CollectionId::from_str(s.as_str())?))
        .collect::<anyhow::Result<Vec<CollectionId>>>()
}

async fn list_document_names_all(
    client: &mut MyFirestoreClient,
    collection_name: &CollectionName,
) -> anyhow::Result<Vec<DocumentName>> {
    let mut document_names = vec![];
    let mut next_page_token = String::new();
    loop {
        let response = client
            .list_documents(ListDocumentsRequest {
                collection_id: collection_name.collection_id().to_string(),
                mask: Some(DocumentMask {
                    field_paths: vec!["__name__".to_string()],
                }),
                page_token: next_page_token.clone(),
                parent: collection_name
                    .parent()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| collection_name.root_document_name().to_string()),
                ..Default::default()
            })
            .await?
            .into_inner();
        document_names.extend(response.documents.into_iter().map(|d| d.name));
        if response.next_page_token.is_empty() {
            break;
        }
        next_page_token = response.next_page_token;
    }
    document_names
        .into_iter()
        .map(|s| Ok(DocumentName::from_str(&s)?))
        .collect::<anyhow::Result<Vec<DocumentName>>>()
}
