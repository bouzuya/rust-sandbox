use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::bail;
use google_cloud_auth::{Credential, CredentialConfig};
use tonic::{
    codegen::InterceptedService,
    metadata::AsciiMetadataValue,
    transport::{Channel, ClientTlsConfig, Endpoint},
    Code, Request, Status,
};

use crate::{
    domain::{MyTweet, ScheduledTweet},
    google::firestore::v1::{
        firestore_client::FirestoreClient, precondition::ConditionType, value::ValueType,
        write::Operation, CommitRequest, Document, GetDocumentRequest, Precondition, Value, Write,
    },
};

#[derive(Debug)]
pub struct TweetStore {
    path: PathBuf,
}

impl Default for TweetStore {
    fn default() -> Self {
        let path = Path::new(&env::var("HOME").expect("env HOME")).join("twiq-light.json");
        Self { path }
    }
}

impl TweetStore {
    pub fn read_all(&self) -> anyhow::Result<BTreeMap<String, MyTweet>> {
        if !self.path().exists() {
            Ok(BTreeMap::new())
        } else {
            let s = fs::read_to_string(self.path())?;
            Ok(serde_json::from_str(&s)?)
        }
    }

    pub fn write_all(&self, data: &BTreeMap<String, MyTweet>) -> anyhow::Result<()> {
        Ok(fs::write(self.path(), serde_json::to_string(data)?)?)
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

// firestore

#[derive(Debug)]
pub struct TweetQueueStore;

impl Default for TweetQueueStore {
    fn default() -> Self {
        Self
    }
}

type MyInterceptor = Box<dyn Fn(Request<()>) -> Result<Request<()>, Status>>;
type Client = FirestoreClient<InterceptedService<Channel, MyInterceptor>>;

impl TweetQueueStore {
    pub async fn read_all(&self) -> anyhow::Result<VecDeque<ScheduledTweet>> {
        let mut client = Self::get_client().await?;
        let document_path = Self::get_document_path()?;
        let document = Self::get_document(&mut client, &document_path).await?;
        Ok(match document {
            Some(doc) => {
                let value = doc.fields.get("data").expect("data field not found");
                match value.value_type.as_ref() {
                    Some(value_type) => match value_type {
                        ValueType::StringValue(s) => serde_json::from_str(s.as_str())?,
                        _ => unreachable!("data field value_type is not string"),
                    },
                    None => unreachable!(),
                }
            }
            None => VecDeque::default(),
        })
    }

    pub async fn write_all(&self, data: &VecDeque<ScheduledTweet>) -> anyhow::Result<()> {
        let s = serde_json::to_string(&data)?;
        // <https://cloud.google.com/firestore/quotas>
        // Maximum size of a field value: 1 MiB - 89 bytes (1,048,487 bytes)
        let byte_length = s.len();
        if byte_length > 1_000_000 {
            bail!("Maximum field size exceeded");
        }

        let mut client = Self::get_client().await?;
        let database_path = Self::get_database_path()?;
        let document_path = Self::get_document_path()?;
        let document = Self::get_document(&mut client, &document_path).await?;
        let condition_type = match document {
            Some(doc) => {
                let update_time = doc.update_time.expect("output contains update_time");
                ConditionType::UpdateTime(update_time)
            }
            None => ConditionType::Exists(false),
        };
        let document = Document {
            name: document_path,
            fields: {
                let mut fields = HashMap::new();
                fields.insert(
                    "data".to_owned(),
                    Value {
                        value_type: Some(ValueType::StringValue(s)),
                    },
                );
                fields
            },
            create_time: None,
            update_time: None,
        };
        let writes = vec![Write {
            update_mask: None,
            update_transforms: vec![],
            current_document: Some(Precondition {
                condition_type: Some(condition_type),
            }),
            operation: Some(Operation::Update(document)),
        }];
        client
            .commit(CommitRequest {
                database: database_path,
                writes,
                transaction: vec![],
            })
            .await?;
        Ok(())
    }

    async fn get_client() -> anyhow::Result<Client> {
        // GOOGLE_APPLICATION_CREDENTIALS environment variable
        let config = CredentialConfig::builder()
            .scopes(vec!["https://www.googleapis.com/auth/cloud-platform".into()])
            .build()?;
        let credential = Credential::find_default(config).await?;
        let channel = Endpoint::from_static("https://firestore.googleapis.com")
            .tls_config(ClientTlsConfig::new().domain_name("firestore.googleapis.com"))?
            .connect()
            .await?;
        let access_token = credential.access_token().await?;
        let mut metadata_value =
            AsciiMetadataValue::try_from(format!("Bearer {}", access_token.value))?;
        metadata_value.set_sensitive(true);
        let client: FirestoreClient<InterceptedService<Channel, MyInterceptor>> =
            FirestoreClient::with_interceptor(
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

    fn get_database_path() -> anyhow::Result<String> {
        let project_id = env::var("PROJECT_ID")?;
        let database_id = "(default)";
        let database_path = format!("projects/{}/databases/{}", project_id, database_id);
        Ok(database_path)
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

    fn get_document_path() -> anyhow::Result<String> {
        let database_path = Self::get_database_path()?;
        let collection_id = "twiq-light";
        let document_id = "queue";
        let document_path = format!(
            "{}/documents/{}/{}",
            database_path, collection_id, document_id
        );
        Ok(document_path)
    }
}

// fs

// #[derive(Debug)]
// pub struct TweetQueueStore {
//     path: PathBuf,
// }

// impl Default for TweetQueueStore {
//     fn default() -> Self {
//         let path = Path::new(&env::var("HOME").expect("env HOME")).join("twiq-light-queue.json");
//         Self { path }
//     }
// }

// impl TweetQueueStore {
//     pub async fn read_all(&self) -> anyhow::Result<VecDeque<ScheduledTweet>> {
//         if !self.path().exists() {
//             Ok(VecDeque::new())
//         } else {
//             let s = fs::read_to_string(self.path())?;
//             Ok(serde_json::from_str(&s)?)
//         }
//     }

//     pub async fn write_all(&self, data: &VecDeque<ScheduledTweet>) -> anyhow::Result<()> {
//         Ok(fs::write(self.path(), serde_json::to_string(data)?)?)
//     }

//     fn path(&self) -> &Path {
//         &self.path
//     }
// }
