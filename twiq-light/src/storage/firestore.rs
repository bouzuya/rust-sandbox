use std::collections::HashMap;

use anyhow::bail;
use async_trait::async_trait;
use google_cloud_auth::{Credential, CredentialConfig};
use tonic::{
    codegen::InterceptedService,
    metadata::AsciiMetadataValue,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Code, Request, Status,
};

use crate::google::firestore::v1::{
    firestore_client::FirestoreClient, value::ValueType, DeleteDocumentRequest, Document,
    DocumentMask, GetDocumentRequest, ListDocumentsRequest, UpdateDocumentRequest, Value,
};

use super::Storage;

pub struct FirestoreStorage {
    credential: Credential,
    project_id: String,
    database_id: String,
    collection_id: String,
}

type MyInterceptor = Box<dyn Fn(Request<()>) -> Result<Request<()>, Status> + Send>;
type Client = FirestoreClient<InterceptedService<Channel, MyInterceptor>>;

impl FirestoreStorage {
    const FIELD_NAME: &str = "data";

    // GOOGLE_APPLICATION_CREDENTIALS environment variable
    pub async fn new(
        google_application_credentials: Option<String>,
        project_id: String,
        database_id: String,
        collection_id: String,
    ) -> anyhow::Result<Self> {
        let config = CredentialConfig::builder()
            .scopes(vec!["https://www.googleapis.com/auth/cloud-platform".into()])
            .build()?;
        let credential = match google_application_credentials.as_ref() {
            Some(file_path) => Credential::find(file_path, config).await?,
            None => Credential::find_default(config).await?,
        };
        Ok(Self {
            credential,
            project_id,
            database_id,
            collection_id,
        })
    }

    async fn get_client(&self) -> anyhow::Result<Client> {
        let channel = Endpoint::from_static("https://firestore.googleapis.com")
            .tls_config(ClientTlsConfig::new().domain_name("firestore.googleapis.com"))?
            .connect()
            .await?;
        let access_token = self.credential.access_token().await?;
        let mut metadata_value =
            AsciiMetadataValue::try_from(format!("Bearer {}", access_token.value))?;
        metadata_value.set_sensitive(true);
        let client: Client = FirestoreClient::with_interceptor(
            channel,
            Box::new(move |mut request: Request<()>| {
                request
                    .metadata_mut()
                    .insert("authorization", metadata_value.clone());
                Ok(request)
            }),
        );
        Ok(client)
    }

    async fn get_document(
        client: &mut Client,
        document_path: &str,
    ) -> anyhow::Result<Option<Document>> {
        let document = client
            .get_document(GetDocumentRequest {
                name: document_path.to_owned(),
                mask: None,
                consistency_selector: None,
            })
            .await
            .map(|response| Some(response.into_inner()))
            .or_else(|status| {
                if matches!(status.code(), Code::NotFound) {
                    Ok(None)
                } else {
                    Err(status)
                }
            })?;
        Ok(document)
    }

    fn data_from_document(document: &Document) -> &str {
        let value = document
            .fields
            .get(Self::FIELD_NAME)
            .expect("field not found");
        match value.value_type.as_ref() {
            Some(value_type) => match value_type {
                ValueType::StringValue(s) => s.as_str(),
                _ => unreachable!("value_type is not string"),
            },
            None => unreachable!(),
        }
    }

    fn document_from_data(document_path: String, s: String) -> Document {
        Document {
            name: document_path,
            fields: {
                let mut fields = HashMap::new();
                fields.insert(
                    Self::FIELD_NAME.to_owned(),
                    Value {
                        value_type: Some(ValueType::StringValue(s)),
                    },
                );
                fields
            },
            create_time: None,
            update_time: None,
        }
    }
}

#[async_trait]
impl Storage for FirestoreStorage {
    type Key = String;

    type Value = String;

    async fn get_item(&self, key: Self::Key) -> anyhow::Result<Option<Self::Value>> {
        let mut client = self.get_client().await?;
        let name = format!(
            "projects/{}/databases/{}/documents/{}/{}",
            self.project_id, self.database_id, self.collection_id, key
        );
        let document = Self::get_document(&mut client, name.as_ref()).await?;
        Ok(document.map(|doc| Self::data_from_document(&doc).to_owned()))
    }

    async fn keys(&self) -> anyhow::Result<Vec<Self::Key>> {
        let mut client = self.get_client().await?;
        let parent = format!(
            "projects/{}/databases/{}/documents",
            self.project_id, self.database_id
        );
        let collection_id = self.collection_id.clone();
        let prefix = format!("{parent}/{collection_id}/");
        let response = client
            .list_documents(ListDocumentsRequest {
                parent,
                collection_id,
                page_size: 100,
                page_token: "".to_owned(),
                order_by: "".to_owned(),
                mask: Some(DocumentMask {
                    field_paths: vec!["name".to_owned()],
                }),
                show_missing: false,
                consistency_selector: None,
            })
            .await?;
        Ok(response
            .into_inner()
            .documents
            .into_iter()
            .map(|doc| doc.name.trim_start_matches(prefix.as_str()).to_owned())
            .collect::<Vec<String>>())
    }

    async fn remove_item(&self, key: Self::Key) -> anyhow::Result<()> {
        let mut client = self.get_client().await?;
        let name = format!(
            "projects/{}/databases/{}/documents/{}/{}",
            self.project_id, self.database_id, self.collection_id, key
        );
        client
            .delete_document(DeleteDocumentRequest {
                name,
                current_document: None,
            })
            .await?;
        Ok(())
    }

    async fn set_item(&self, key: Self::Key, value: Self::Value) -> anyhow::Result<()> {
        // <https://cloud.google.com/firestore/quotas>
        // Maximum size of a field value: 1 MiB - 89 bytes (1,048,487 bytes)
        let byte_length = value.len();
        if byte_length > 1_000_000 {
            bail!("Maximum field size exceeded");
        }

        let name = format!(
            "projects/{}/databases/{}/documents/{}/{}",
            self.project_id, self.database_id, self.collection_id, key
        );
        let document = Self::document_from_data(name, value);
        let mut client = self.get_client().await?;
        client
            .update_document(UpdateDocumentRequest {
                document: Some(document),
                update_mask: None,
                mask: None,
                current_document: None,
            })
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let storage = FirestoreStorage::new(
            Some(env::var("TWIQ_LIGHT_GOOGLE_APPLICATION_CREDENTIALS")?),
            env::var("TWIQ_LIGHT_GOOGLE_PROJECT_ID")?,
            "(default)".to_owned(),
            "firestore-storage-test".to_owned(),
        )
        .await?;

        let key1: String = "key1".to_owned();
        let val1: String = "value1".to_owned();

        assert!(storage.keys().await?.is_empty());
        assert_eq!(storage.get_item(key1.clone()).await?, None);

        storage.set_item(key1.clone(), val1.clone()).await?;
        assert_eq!(storage.keys().await?, vec![key1.clone()]);
        assert_eq!(storage.get_item(key1.clone()).await?, Some(val1));

        storage.remove_item(key1.clone()).await?;
        assert!(storage.keys().await?.is_empty());
        assert_eq!(storage.get_item(key1.clone()).await?, None);
        Ok(())
    }
}
