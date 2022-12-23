use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    env,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{bail, Context};
use google_cloud_auth::{Credential, CredentialConfig};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
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
    twitter::AccessTokenResponse,
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Token {
    pub access_token: String,
    pub expires: String,
    pub refresh_token: String,
}

impl FromStr for Token {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).expect("to_string"))
    }
}

impl Token {
    pub fn try_from(
        access_token_response: AccessTokenResponse,
        unix_timestamp: i64,
    ) -> anyhow::Result<Token> {
        let now = OffsetDateTime::from_unix_timestamp(unix_timestamp)?;

        let access_token = access_token_response.access_token;
        let expires_in = access_token_response
            .expires_in
            .context("expires_in is none")?;
        let refresh_token = access_token_response
            .refresh_token
            .context("refresh_token is none")?;

        let expires = now + Duration::seconds(i64::from(expires_in));
        let expires = expires.format(&Rfc3339)?;
        Ok(Token {
            access_token,
            expires,
            refresh_token,
        })
    }
}

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
pub struct TweetQueueStore {
    project_id: String,
    google_application_credentials: Option<PathBuf>,
}

type MyInterceptor = Box<dyn Fn(Request<()>) -> Result<Request<()>, Status>>;
type Client = FirestoreClient<InterceptedService<Channel, MyInterceptor>>;

impl TweetQueueStore {
    const DATABASE_ID: &str = "(default)";
    const COLLECTION_ID: &str = "twiq-light";
    const DOCUMENT_ID: &str = "queue";
    const FIELD_NAME: &str = "data";
    const TOKEN_DOCUMENT_ID: &str = "token";

    pub fn new(google_application_credentials: Option<String>) -> Self {
        let project_id = env::var("PROJECT_ID").expect("PROJECT_ID");
        Self {
            project_id,
            google_application_credentials: google_application_credentials.map(PathBuf::from),
        }
    }

    pub async fn read_all(&self) -> anyhow::Result<VecDeque<ScheduledTweet>> {
        let mut client = self.get_client().await?;
        let document_path = self.get_document_path()?;
        let document = Self::get_document(&mut client, &document_path).await?;
        Ok(match document {
            Some(doc) => serde_json::from_str(Self::data_from_document(&doc))?,
            None => VecDeque::default(),
        })
    }

    pub async fn read_token(&self) -> anyhow::Result<Option<Token>> {
        let mut client = self.get_client().await?;
        let document_path = self.get_token_document_path()?;
        let document = Self::get_document(&mut client, &document_path).await?;
        Ok(match document {
            Some(doc) => Some(serde_json::from_str(Self::data_from_document(&doc))?),
            None => None,
        })
    }

    pub async fn write_token(&self, token: &Token) -> anyhow::Result<()> {
        let s = token.to_string();

        // <https://cloud.google.com/firestore/quotas>
        // Maximum size of a field value: 1 MiB - 89 bytes (1,048,487 bytes)
        let byte_length = s.len();
        if byte_length > 1_000_000 {
            bail!("Maximum field size exceeded");
        }

        let mut client = self.get_client().await?;
        let database_path = self.get_database_path()?;
        let document_path = self.get_token_document_path()?;
        let document = Self::get_document(&mut client, &document_path).await?;
        let condition_type = match document {
            Some(doc) => {
                let update_time = doc.update_time.expect("output contains update_time");
                ConditionType::UpdateTime(update_time)
            }
            None => ConditionType::Exists(false),
        };
        let document = Self::document_from_data(document_path, s);
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

    pub async fn write_all(&self, data: &VecDeque<ScheduledTweet>) -> anyhow::Result<()> {
        let s = serde_json::to_string(&data)?;
        // <https://cloud.google.com/firestore/quotas>
        // Maximum size of a field value: 1 MiB - 89 bytes (1,048,487 bytes)
        let byte_length = s.len();
        if byte_length > 1_000_000 {
            bail!("Maximum field size exceeded");
        }

        let mut client = self.get_client().await?;
        let database_path = self.get_database_path()?;
        let document_path = self.get_document_path()?;
        let document = Self::get_document(&mut client, &document_path).await?;
        let condition_type = match document {
            Some(doc) => {
                let update_time = doc.update_time.expect("output contains update_time");
                ConditionType::UpdateTime(update_time)
            }
            None => ConditionType::Exists(false),
        };
        let document = Self::document_from_data(document_path, s);
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

    async fn get_client(&self) -> anyhow::Result<Client> {
        let config = CredentialConfig::builder()
            .scopes(vec!["https://www.googleapis.com/auth/cloud-platform".into()])
            .build()?;
        let credential = match self.google_application_credentials.as_ref() {
            Some(file_path) => Credential::find(file_path, config).await?,
            None => {
                // GOOGLE_APPLICATION_CREDENTIALS environment variable
                Credential::find_default(config).await?
            }
        };
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

    fn get_database_path(&self) -> anyhow::Result<String> {
        let database_path = format!(
            "projects/{}/databases/{}",
            self.project_id,
            Self::DATABASE_ID
        );
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

    fn get_document_path(&self) -> anyhow::Result<String> {
        let database_path = self.get_database_path()?;
        let document_path = format!(
            "{}/documents/{}/{}",
            database_path,
            Self::COLLECTION_ID,
            Self::DOCUMENT_ID
        );
        Ok(document_path)
    }

    fn get_token_document_path(&self) -> anyhow::Result<String> {
        let database_path = self.get_database_path()?;
        let document_path = format!(
            "{}/documents/{}/{}",
            database_path,
            Self::COLLECTION_ID,
            Self::TOKEN_DOCUMENT_ID
        );
        Ok(document_path)
    }
}
