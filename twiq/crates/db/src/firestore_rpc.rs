pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
    pub mod firestore {
        pub mod v1 {
            tonic::include_proto!("google.firestore.v1");
        }
    }
    pub mod r#type {
        tonic::include_proto!("google.r#type");
    }
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

pub mod helper {
    use super::google::firestore::v1::{
        firestore_client::FirestoreClient, value::ValueType, Document, Value,
    };
    use google_cloud_auth::{Credential, CredentialConfig};
    use prost_types::Timestamp;
    use tonic::{
        codegen::InterceptedService, metadata::AsciiMetadataValue, transport::Channel, Request,
        Status,
    };

    pub mod path {
        /// ```rust
        /// # use db::firestore_rpc::helper::path::database_path;
        /// assert_eq!(database_path("1", "2"), "projects/1/databases/2");
        /// ```
        pub fn database_path(project_id: &str, database_id: &str) -> String {
            format!("projects/{}/databases/{}", project_id, database_id)
        }

        /// ```rust
        /// # use db::firestore_rpc::helper::path::documents_path;
        /// assert_eq!(documents_path("1", "2"), "projects/1/databases/2/documents");
        /// ```
        pub fn documents_path(project_id: &str, database_id: &str) -> String {
            format!(
                "projects/{}/databases/{}/documents",
                project_id, database_id
            )
        }

        /// ```rust
        /// # use db::firestore_rpc::helper::path::collection_path;
        /// assert_eq!(collection_path("1", "2", "3"), "projects/1/databases/2/documents/3");
        /// ```
        pub fn collection_path(project_id: &str, database_id: &str, collection_id: &str) -> String {
            format!(
                "projects/{}/databases/{}/documents/{}",
                project_id, database_id, collection_id
            )
        }

        /// ```rust
        /// # use db::firestore_rpc::helper::path::document_path;
        /// assert_eq!(document_path("1", "2", "3", "4"), "projects/1/databases/2/documents/3/4");
        /// ```
        pub fn document_path(
            project_id: &str,
            database_id: &str,
            collection_id: &str,
            document_id: &str,
        ) -> String {
            format!(
                "projects/{}/databases/{}/documents/{}/{}",
                project_id, database_id, collection_id, document_id
            )
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("google_cloud_auth {0}")]
        GoogleCloudAuth(#[from] google_cloud_auth::Error),
        #[error("tonic invalid metadata value {0}")]
        TonicInvalidMetadataValue(#[from] tonic::metadata::errors::InvalidMetadataValue),
        #[error("tonic status {0}")]
        TonicStatus(#[from] tonic::Status),
        #[error("tonic transport {0}")]
        TonicTransport(#[from] tonic::transport::Error),
    }

    pub type Result<T, E = Error> = std::result::Result<T, E>;

    pub async fn client(
        credential: &Credential,
        channel: Channel,
    ) -> Result<
        FirestoreClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    > {
        let access_token = credential.access_token().await?;
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

    pub async fn credential() -> Result<Credential> {
        // GOOGLE_APPLICATION_CREDENTIALS environment variable
        let config = CredentialConfig::builder()
            .scopes(vec!["https://www.googleapis.com/auth/cloud-platform".into()])
            .build()?;
        Ok(Credential::find_default(config).await?)
    }

    #[derive(Debug, thiserror::Error)]
    pub enum GetFieldError {
        #[error("invalid value type {field} {expected} {actual}")]
        InvalidValueType {
            actual: String,
            expected: String,
            field: String,
        },
        #[error("key not found {0}")]
        KeyNotFound(String),
    }

    pub fn get_field_as_i64(document: &Document, key: &str) -> Result<i64, GetFieldError> {
        get_field(document, key, value_as_i64)
    }

    pub fn get_field_as_str<'a, 'b>(
        document: &'a Document,
        key: &'b str,
    ) -> Result<&'a str, GetFieldError> {
        get_field(document, key, value_as_str)
    }

    pub fn get_field_as_timestamp(
        document: &Document,
        key: &str,
    ) -> Result<Timestamp, GetFieldError> {
        get_field(document, key, value_as_timestamp)
    }

    pub fn get_field<'a, 'b, F, T>(
        document: &'a Document,
        key: &'b str,
        f: F,
    ) -> Result<T, GetFieldError>
    where
        F: Fn(&'b str, &'a Value) -> Result<T, GetFieldError>,
    {
        document
            .fields
            .get(key)
            .ok_or_else(|| GetFieldError::KeyNotFound(key.to_owned()))
            .and_then(|value| f(key, value))
    }

    fn value_type_string(value_type: &ValueType) -> String {
        match value_type {
            ValueType::NullValue(_) => "nullValue",
            ValueType::BooleanValue(_) => "booleanValue",
            ValueType::IntegerValue(_) => "integerValue",
            ValueType::DoubleValue(_) => "doubleValue",
            ValueType::TimestampValue(_) => "timestampValue",
            ValueType::StringValue(_) => "stringValue",
            ValueType::BytesValue(_) => "bytesValue",
            ValueType::ReferenceValue(_) => "referenceValue",
            ValueType::GeoPointValue(_) => "geoPointValue",
            ValueType::ArrayValue(_) => "arrayValue",
            ValueType::MapValue(_) => "mapValue",
        }
        .to_owned()
    }

    fn value_as_i64(field: &str, value: &Value) -> Result<i64, GetFieldError> {
        match value.value_type.as_ref() {
            Some(value_type) => match value_type {
                ValueType::IntegerValue(i) => Ok(*i),
                _ => Err(GetFieldError::InvalidValueType {
                    actual: value_type_string(value_type),
                    expected: value_type_string(&ValueType::IntegerValue(Default::default())),
                    field: field.to_owned(),
                }),
            },
            None => unreachable!(),
        }
    }

    fn value_as_str<'a, 'b>(field: &'a str, value: &'b Value) -> Result<&'b str, GetFieldError> {
        match value.value_type.as_ref() {
            Some(value_type) => match value_type {
                ValueType::StringValue(s) => Ok(s.as_str()),
                _ => Err(GetFieldError::InvalidValueType {
                    actual: value_type_string(value_type),
                    expected: value_type_string(&ValueType::StringValue(Default::default())),
                    field: field.to_owned(),
                }),
            },
            None => unreachable!(),
        }
    }

    fn value_as_timestamp(field: &str, value: &Value) -> Result<Timestamp, GetFieldError> {
        match value.value_type.as_ref() {
            Some(value_type) => match value_type {
                ValueType::TimestampValue(t) => Ok(t.clone()),
                _ => Err(GetFieldError::InvalidValueType {
                    actual: value_type_string(value_type),
                    expected: value_type_string(&ValueType::StringValue(Default::default())),
                    field: field.to_owned(),
                }),
            },
            None => unreachable!(),
        }
    }

    // panic if value_type is not string
    pub fn value_as_str_unchecked(value: &Value) -> &str {
        match value.value_type.as_ref() {
            Some(ValueType::StringValue(s)) => s.as_str(),
            _ => unreachable!(),
        }
    }

    pub fn value_from_i64(i: i64) -> Value {
        Value {
            value_type: Some(ValueType::IntegerValue(i)),
        }
    }

    pub fn value_from_string(s: String) -> Value {
        Value {
            value_type: Some(ValueType::StringValue(s)),
        }
    }

    pub fn value_from_timestamp(t: Timestamp) -> Value {
        Value {
            value_type: Some(ValueType::TimestampValue(t)),
        }
    }

    // panic if value_type is not integer
    pub fn value_into_i64_unchecked(value: &Value) -> i64 {
        match value.value_type {
            Some(ValueType::IntegerValue(i)) => i,
            _ => unreachable!(),
        }
    }

    // panic if value_type is not string
    pub fn value_into_string_unchecked(value: Value) -> String {
        match value.value_type {
            Some(ValueType::StringValue(s)) => s,
            _ => unreachable!(),
        }
    }

    // panic if value_type is not string
    pub fn value_to_timestamp_unchecked(value: &Value) -> Timestamp {
        match value.value_type.as_ref() {
            Some(ValueType::TimestampValue(t)) => t.clone(),
            _ => unreachable!(),
        }
    }

    #[cfg(test)]
    mod tests {
        use std::{collections::HashMap, str::FromStr};

        use crate::firestore_rpc::google::firestore::v1::Document;

        use super::*;

        #[test]
        fn get_field_as_i64_test() -> anyhow::Result<()> {
            assert_eq!(
                get_field_as_i64(
                    &Document {
                        name: "name".to_owned(),
                        fields: {
                            let mut fields = HashMap::new();
                            fields.insert("key".to_owned(), value_from_i64(123));
                            fields
                        },
                        create_time: None,
                        update_time: None
                    },
                    "key"
                )?,
                123
            );
            Ok(())
        }

        #[test]
        fn get_field_as_str_test() -> anyhow::Result<()> {
            assert_eq!(
                get_field_as_str(
                    &Document {
                        name: "name".to_owned(),
                        fields: {
                            let mut fields = HashMap::new();
                            fields.insert("key".to_owned(), value_from_string("val".to_owned()));
                            fields
                        },
                        create_time: None,
                        update_time: None
                    },
                    "key"
                )?,
                "val"
            );
            Ok(())
        }

        #[test]
        fn get_field_as_timestamp_test() -> anyhow::Result<()> {
            let timestamp = Timestamp::from_str("2020-01-02T15:04:05Z")?;
            assert_eq!(
                get_field_as_timestamp(
                    &Document {
                        name: "name".to_owned(),
                        fields: {
                            let mut fields = HashMap::new();
                            fields
                                .insert("key".to_owned(), value_from_timestamp(timestamp.clone()));
                            fields
                        },
                        create_time: None,
                        update_time: None
                    },
                    "key"
                )?,
                timestamp
            );
            Ok(())
        }

        #[test]
        fn value_as_str_unchecked_test() {
            assert_eq!(
                value_as_str_unchecked(&Value {
                    value_type: Some(ValueType::StringValue("abc".to_owned())),
                }),
                "abc"
            );
        }

        #[test]
        fn value_from_i64_test() {
            assert_eq!(
                value_from_i64(123),
                Value {
                    value_type: Some(ValueType::IntegerValue(123))
                }
            );
        }

        #[test]
        fn value_from_string_test() {
            assert_eq!(
                value_from_string("abc".to_owned()),
                Value {
                    value_type: Some(ValueType::StringValue("abc".to_owned()))
                }
            );
        }

        #[test]
        fn value_from_timestamp_test() -> anyhow::Result<()> {
            let timestamp = Timestamp::from_str("2020-01-02T15:04:05Z")?;
            assert_eq!(
                value_from_timestamp(timestamp.clone()),
                Value {
                    value_type: Some(ValueType::TimestampValue(timestamp))
                }
            );
            Ok(())
        }

        #[test]
        fn value_into_i64_unchecked_test() {
            assert_eq!(
                value_into_i64_unchecked(&Value {
                    value_type: Some(ValueType::IntegerValue(123)),
                }),
                123
            );
        }

        #[test]
        fn value_into_string_unchecked_test() {
            assert_eq!(
                value_into_string_unchecked(Value {
                    value_type: Some(ValueType::StringValue("abc".to_owned())),
                }),
                "abc".to_owned()
            );
        }

        #[test]
        fn value_to_timestamp_unchecked_test() -> anyhow::Result<()> {
            let timestamp = Timestamp::from_str("2020-01-02T15:04:05Z")?;
            assert_eq!(
                value_to_timestamp_unchecked(&Value {
                    value_type: Some(ValueType::TimestampValue(timestamp.clone())),
                }),
                timestamp
            );
            Ok(())
        }
    }
}
