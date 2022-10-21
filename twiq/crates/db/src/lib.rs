pub mod firestore_event_store;
pub mod firestore_rest;
pub mod firestore_rpc;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use google_cloud_auth::{Credential, CredentialConfig};
    use tonic::{
        metadata::{Ascii, MetadataValue},
        transport::{ClientTlsConfig, Endpoint},
        Request,
    };

    use crate::firestore_rpc::google::firestore::v1::{
        firestore_client::FirestoreClient, value::ValueType, CreateDocumentRequest, Document, Value,
    };

    #[tokio::test]
    #[ignore]
    async fn create_document_test() -> anyhow::Result<()> {
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let parent = format!(
            "projects/{}/databases/{}/documents",
            project_id, database_id,
        );
        let collection_id = "cities".to_owned();
        let document_id = "LA".to_owned();
        let document = Document {
            name: "".to_owned(),
            fields: {
                let mut map = HashMap::new();
                map.insert(
                    "name".to_string(),
                    Value {
                        value_type: Some(ValueType::StringValue("Los Angeles".to_string())),
                    },
                );
                map.insert(
                    "state".to_string(),
                    Value {
                        value_type: Some(ValueType::StringValue("CA".to_string())),
                    },
                );
                map.insert(
                    "country".to_string(),
                    Value {
                        value_type: Some(ValueType::StringValue("USA".to_string())),
                    },
                );
                map
            },
            create_time: None,
            update_time: None,
        };

        let credential = credential().await?;
        let access_token = credential.access_token().await?;

        let channel = Endpoint::from_static("https://firestore.googleapis.com")
            .tls_config(ClientTlsConfig::new().domain_name("firestore.googleapis.com"))?
            .connect()
            .await?;

        let mut firestore_client =
            FirestoreClient::with_interceptor(channel, |mut req: Request<_>| {
                req.metadata_mut().insert("authorization", {
                    let mut metadata_value: MetadataValue<Ascii> =
                        format!("Bearer {}", access_token.value).parse().unwrap();
                    metadata_value.set_sensitive(true);
                    metadata_value
                });
                Ok(req)
            });

        let response = firestore_client
            .create_document(CreateDocumentRequest {
                parent,
                collection_id,
                document_id,
                document: Some(document),
                mask: None,
            })
            .await?;
        assert_eq!("", format!("{:?}", response));
        Ok(())
    }

    async fn credential() -> anyhow::Result<Credential> {
        let config = CredentialConfig::builder()
            .scopes(vec!["https://www.googleapis.com/auth/cloud-platform".into()])
            .build()?;
        Ok(Credential::find_default(config).await?)
    }
}
