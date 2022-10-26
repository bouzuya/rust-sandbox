pub mod firestore_rest;
pub mod firestore_rest_event_store;
pub mod firestore_rpc;
pub mod firestore_rpc_event_store;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, str::FromStr};

    use domain::aggregate::user::{TwitterUserId, User};
    use event_store_core::{event_store::EventStore, EventStream};
    use google_cloud_auth::{Credential, CredentialConfig};
    use prost_types::Timestamp;
    use time::OffsetDateTime;
    use tonic::{
        codegen::InterceptedService,
        metadata::AsciiMetadataValue,
        transport::{Channel, ClientTlsConfig, Endpoint},
        Request, Status,
    };

    use crate::{
        firestore_rpc::google::firestore::v1::{
            firestore_client::FirestoreClient,
            get_document_request::ConsistencySelector,
            precondition::ConditionType,
            transaction_options::{Mode, ReadWrite},
            value::ValueType,
            write::Operation,
            BeginTransactionRequest, CommitRequest, CreateDocumentRequest, Document,
            GetDocumentRequest, Precondition, TransactionOptions, Value, Write,
        },
        firestore_rpc_event_store::FirestoreRpcEventStore,
    };

    #[tokio::test]
    #[ignore]
    async fn firestore_rpc_event_store_test() -> anyhow::Result<()> {
        let credential = credential().await?;
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)".to_owned();
        let transaction =
            FirestoreRpcEventStore::begin_transaction(&credential, &project_id, &database_id)
                .await?;
        let event_store = FirestoreRpcEventStore::new(
            credential.clone(),
            project_id.clone(),
            database_id.clone(),
            transaction.clone(),
        );

        let user = User::create(TwitterUserId::from_str("125962981")?)?;
        event_store.store(None, EventStream::from(user)).await?;

        let writes = event_store.writes().await;
        FirestoreRpcEventStore::commit(&credential, &project_id, &database_id, transaction, writes)
            .await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn begin_transaction_test() -> anyhow::Result<()> {
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let database = format!("projects/{}/databases/{}", project_id, database_id);

        let credential = credential().await?;
        let mut client = client(&credential).await?;
        let response = client
            .begin_transaction(BeginTransactionRequest {
                database,
                options: Some(TransactionOptions {
                    mode: Some(Mode::ReadWrite(ReadWrite {
                        retry_transaction: vec![],
                    })),
                }),
            })
            .await?;
        assert_eq!("", format!("{:?}", response));
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn commit_test() -> anyhow::Result<()> {
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let database = format!("projects/{}/databases/{}", project_id, database_id);

        // begin_transaction
        let credential = credential().await?;
        let mut client = client(&credential).await?;
        let response = client
            .begin_transaction(BeginTransactionRequest {
                database: database.clone(),
                options: Some(TransactionOptions {
                    mode: Some(Mode::ReadWrite(ReadWrite {
                        retry_transaction: vec![],
                    })),
                }),
            })
            .await?;
        let transaction = response.into_inner().transaction;

        // commit
        let collection_id = "cities".to_owned();
        let document_id = "LA".to_owned();
        let mut document = build_document();
        document.name = format!(
            "projects/{}/databases/{}/documents/{}/{}",
            &project_id, &database_id, collection_id, document_id
        );
        let response = client
            .commit(CommitRequest {
                database,
                writes: vec![Write {
                    update_mask: None,
                    update_transforms: vec![],
                    current_document: Some(Precondition {
                        condition_type: Some(ConditionType::Exists(false)),
                    }),
                    operation: Some(Operation::Update(document)),
                }],
                transaction,
            })
            .await?;
        assert_eq!("", format!("{:?}", response));
        Ok(())
    }

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
        let document = build_document();

        let credential = credential().await?;
        let mut client = client(&credential).await?;
        let response = client
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

    #[tokio::test]
    #[ignore]
    async fn get_test() -> anyhow::Result<()> {
        let now = {
            let odt = OffsetDateTime::now_utc();
            Timestamp::date_time_nanos(
                i64::from(odt.year()),
                u8::from(odt.month()),
                odt.day(),
                odt.hour(),
                odt.minute(),
                odt.second(),
                odt.nanosecond(),
            )?
        };
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let collection_id = "cities".to_owned();
        let document_id = "LA".to_owned();
        let document_path = format!("{}/{}", collection_id, document_id);
        let name = format!(
            "projects/{}/databases/{}/documents/{}",
            project_id, database_id, document_path
        );
        let credential = credential().await?;
        let mut client = client(&credential).await?;
        let response = client
            .get_document(GetDocumentRequest {
                name,
                mask: None,
                consistency_selector: Some(ConsistencySelector::ReadTime(now)),
            })
            .await?;
        assert_eq!("", format!("{:?}", response));
        Ok(())
    }

    fn build_document() -> Document {
        Document {
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
        }
    }

    async fn client(
        credential: &Credential,
    ) -> anyhow::Result<
        FirestoreClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    > {
        let access_token = credential.access_token().await?;
        let channel = Endpoint::from_static("https://firestore.googleapis.com")
            .tls_config(ClientTlsConfig::new().domain_name("firestore.googleapis.com"))?
            .connect()
            .await?;
        let mut metadata_value =
            AsciiMetadataValue::try_from(format!("Bearer {}", access_token.value))?;
        metadata_value.set_sensitive(true);
        let client = FirestoreClient::with_interceptor(channel, move |mut request: Request<()>| {
            request
                .metadata_mut()
                .insert("authorization", metadata_value.clone());
            Ok(request)
        });
        Ok(client)
    }

    async fn credential() -> anyhow::Result<Credential> {
        let config = CredentialConfig::builder()
            .scopes(vec!["https://www.googleapis.com/auth/cloud-platform".into()])
            .build()?;
        Ok(Credential::find_default(config).await?)
    }
}
