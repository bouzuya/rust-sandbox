use std::collections::HashMap;

use ordered_float::NotNan;
use reqwest::{Client, Method, Response, Url};
use serde::{de::Visitor, ser::SerializeMap};

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Double(NotNan<f64>),
    Timestamp(Timestamp),
    String(String),
    Bytes(String),     // base64-encoded string
    Reference(String), // e.g. projects/{project_id}/databases/{databaseId}/documents/{document_path}
    GeoPoint(LatLng),
    Array(Array),
    Map(Map),
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("deserialize value")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let key: Option<String> = map.next_key()?;
        match key {
            Some(k) => Ok(match k.as_str() {
                "nullValue" => {
                    let v = map.next_value::<serde_json::Value>()?;
                    if v.is_null() {
                        Value::Null
                    } else {
                        return Err(serde::de::Error::invalid_type(
                            serde::de::Unexpected::Map,
                            &self,
                        ));
                    }
                }
                "booleanValue" => Value::Boolean(map.next_value()?),
                "integerValue" => {
                    let v: String = map.next_value()?;
                    match v.parse::<i64>() {
                        Ok(v) => Value::Integer(v),
                        Err(_) => {
                            return Err(serde::de::Error::invalid_type(
                                serde::de::Unexpected::Map,
                                &self,
                            ))
                        }
                    }
                }
                "doubleValue" => {
                    let v: f64 = map.next_value()?;
                    match NotNan::new(v) {
                        Ok(v) => Value::Double(v),
                        Err(_) => {
                            return Err(serde::de::Error::invalid_type(
                                serde::de::Unexpected::Map,
                                &self,
                            ))
                        }
                    }
                }
                "timestampValue" => Value::Timestamp(map.next_value()?),
                "stringValue" => Value::String(map.next_value()?),
                "bytesValue" => Value::Bytes(map.next_value()?),
                "referenceValue" => Value::Reference(map.next_value()?),
                "geoPointValue" => Value::GeoPoint(map.next_value()?),
                "arrayValue" => Value::Array(map.next_value()?),
                "mapValue" => Value::Map(map.next_value()?),
                _ => unimplemented!(),
            }),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Map,
                &self,
            )),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ValueVisitor)
    }
}

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            Value::Null => map.serialize_entry("nullValue", &serde_json::Value::Null)?,
            Value::Boolean(value) => map.serialize_entry("booleanValue", value)?,
            Value::Integer(value) => map.serialize_entry("integerValue", value)?,
            Value::Double(value) => map.serialize_entry("doubleValue", &value.into_inner())?,
            Value::Timestamp(value) => map.serialize_entry("timestampValue", value)?,
            Value::String(value) => map.serialize_entry("stringValue", value)?,
            Value::Bytes(value) => map.serialize_entry("bytesValue", value)?,
            Value::Reference(value) => map.serialize_entry("referenceValue", value)?,
            Value::GeoPoint(value) => map.serialize_entry("geoPointValue", value)?,
            Value::Array(value) => map.serialize_entry("arrayValue", value)?,
            Value::Map(value) => map.serialize_entry("mapValue", value)?,
        }
        map.end()
    }
}

pub type Timestamp = String; // 2022-08-19T22:53:42.480950Z

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LatLng {
    pub latitude: NotNan<f64>,
    pub longitude: NotNan<f64>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Array {
    pub values: Vec<Value>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Map {
    pub fields: HashMap<String, Value>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub name: String,
    pub fields: HashMap<String, Value>,
    pub create_time: Timestamp,
    pub update_time: Timestamp,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionOptions {
    ReadOnly {
        read_time: String,
    },
    ReadWrite {
        #[serde(skip_serializing_if = "Option::is_none")]
        retry_transaction: Option<String>,
    },
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BeginTransactionRequestBody {
    pub options: TransactionOptions,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BeginTransactionResponse {
    pub transaction: String,
}

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

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CommitRequestBody {
    writes: Vec<Write>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction: Option<String>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Write {
    // TODO:
    // Update {}
    Delete(String),
    // TODO:
    // Transform {},
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let json = r#"{
            "name": "projects/bouzuya-project/databases/(default)/documents/cities/LA",
            "fields": {
                "name": {
                    "stringValue": "Los Angeles"
                },
                "state": {
                    "stringValue": "CA"
                },
                "country": {
                    "stringValue": "USA"
                }
            },
            "createTime": "2022-08-19T22:53:42.480950Z",
            "updateTime": "2022-08-19T22:53:42.480950Z"
        }"#;
        let document: Document = serde_json::from_str(json)?;
        assert_eq!(
            document.name,
            "projects/bouzuya-project/databases/(default)/documents/cities/LA"
        );
        assert_eq!(document.fields, {
            let mut map = HashMap::new();
            map.insert("name".to_owned(), Value::String("Los Angeles".to_owned()));
            map.insert("state".to_owned(), Value::String("CA".to_owned()));
            map.insert("country".to_owned(), Value::String("USA".to_owned()));
            map
        });
        Ok(())
    }

    #[test]
    fn simple_test() -> anyhow::Result<()> {
        let json = r#"{
            "name": "projects/bouzuya-project/databases/(default)/documents/collection_id/document_id",
            "fields": {
                "null": {
                    "nullValue": null
                },
                "boolean": {
                    "booleanValue": true
                },
                "integer": {
                    "integerValue": "1234"
                },
                "double": {
                    "doubleValue": 123.456
                },
                "timestamp": {
                    "timestampValue": "2001-02-03T04:05:06Z"
                },
                "string": {
                    "stringValue": "s"
                },
                "bytes": {
                    "bytesValue": "Ynl0ZXMK"
                },
                "reference": {
                    "referenceValue": "ref"
                },
                "geoPoint": {
                    "geoPointValue": {
                        "latitude": 123.456,
                        "longitude": 789.012
                    }
                },
                "array": {
                    "arrayValue": {
                        "values": [
                            {
                                "stringValue": "s"
                            }
                        ]
                    }
                },
                "map": {
                    "mapValue": {
                        "fields": {
                            "s": {
                                "stringValue": "s"
                            }
                        }
                    }
                }
            },
            "createTime": "2022-08-19T22:53:42.480950Z",
            "updateTime": "2022-08-19T22:53:42.480950Z"
        }"#;
        let document: Document = serde_json::from_str(json)?;
        assert_eq!(
            document,
            Document {
                name: "projects/bouzuya-project/databases/(default)/documents/collection_id/document_id".to_owned(),
                fields: {
                    let mut map = HashMap::new();
                    map.insert("null".to_owned(), Value::Null);
                    map.insert("boolean".to_owned(), Value::Boolean(true));
                    map.insert("integer".to_owned(), Value::Integer(1234));
                    map.insert("double".to_owned(), Value::Double(NotNan::new(123.456_f64).unwrap()));
                    map.insert("timestamp".to_owned(), Value::Timestamp("2001-02-03T04:05:06Z".to_owned()));
                    map.insert("string".to_owned(), Value::String("s".to_owned()));
                    map.insert("bytes".to_owned(), Value::Bytes("Ynl0ZXMK".to_owned()));
                    map.insert("reference".to_owned(), Value::Reference("ref".to_owned()));
                    map.insert("geoPoint".to_owned(), Value::GeoPoint(LatLng { latitude: NotNan::new(123.456_f64).unwrap(), longitude: NotNan::new(789.012_f64).unwrap() }));
                    map.insert("array".to_owned(), Value::Array(Array { values: vec![Value::String("s".to_owned())] }));
                    map.insert("map".to_owned(), Value::Map(Map { fields: {
                        let mut map = HashMap::new();
                        map.insert("s".to_owned(), Value::String("s".to_owned()));
                        map
                    } }));
                    map
                },
                create_time: "2022-08-19T22:53:42.480950Z".to_owned(),
                update_time: "2022-08-19T22:53:42.480950Z".to_owned()
            }
        );
        assert!(serde_json::to_string(&document).is_ok());
        Ok(())
    }

    #[test]
    fn begin_transaction_request_body_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&BeginTransactionRequestBody {
                options: TransactionOptions::ReadWrite {
                    retry_transaction: None,
                },
            })?,
            r#"{"options":{"readWrite":{}}}"#
        );
        assert_eq!(
            serde_json::to_string(&BeginTransactionRequestBody {
                options: TransactionOptions::ReadWrite {
                    retry_transaction: Some("abc".to_owned()),
                },
            })?,
            r#"{"options":{"readWrite":{"retry_transaction":"abc"}}}"#
        );
        assert_eq!(
            serde_json::to_string(&BeginTransactionRequestBody {
                options: TransactionOptions::ReadOnly {
                    read_time: "2000-01-02T03:04:05Z".to_owned()
                }
            })?,
            r#"{"options":{"readOnly":{"read_time":"2000-01-02T03:04:05Z"}}}"#
        );
        Ok(())
    }

    #[test]
    fn commit_request_body_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&CommitRequestBody {
                writes: vec![Write::Delete("123".to_owned())],
                transaction: None
            })?,
            r#"{"writes":[{"delete":"123"}]}"#
        );
        assert_eq!(
            serde_json::to_string(&CommitRequestBody {
                writes: vec![Write::Delete("123".to_owned())],
                transaction: Some("456".to_owned())
            })?,
            r#"{"writes":[{"delete":"123"}],"transaction":"456"}"#
        );
        Ok(())
    }

    #[test]
    fn write_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&Write::Delete("123".to_owned()))?,
            r#"{"delete":"123"}"#
        );
        Ok(())
    }
}
