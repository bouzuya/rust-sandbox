use ordered_float::NotNan;
use serde::{de::Visitor, ser::SerializeMap};

use super::{Array, LatLng, Map, Timestamp};

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
            Value::Integer(value) => map.serialize_entry("integerValue", &value.to_string())?,
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: Value = serde_json::from_str(r#"{"nullValue":null}"#)?;
        assert_eq!(deserialized, Value::Null);
        let deserialized: Value = serde_json::from_str(r#"{"booleanValue":true}"#)?;
        assert_eq!(deserialized, Value::Boolean(true));
        let deserialized: Value = serde_json::from_str(r#"{"integerValue":"123"}"#)?;
        assert_eq!(deserialized, Value::Integer(123));
        let deserialized: Value = serde_json::from_str(r#"{"doubleValue":123.456}"#)?;
        assert_eq!(deserialized, Value::Double(NotNan::new(123.456)?));
        let deserialized: Value =
            serde_json::from_str(r#"{"timestampValue":"2001-02-03T04:05:06Z"}"#)?;
        assert_eq!(
            deserialized,
            Value::Timestamp("2001-02-03T04:05:06Z".to_owned())
        );
        let deserialized: Value = serde_json::from_str(r#"{"stringValue":"s"}"#)?;
        assert_eq!(deserialized, Value::String("s".to_owned()));
        let deserialized: Value = serde_json::from_str(r#"{"bytesValue":"Ynl0ZXMK"}"#)?;
        assert_eq!(deserialized, Value::Bytes("Ynl0ZXMK".to_owned()));
        let deserialized: Value = serde_json::from_str(r#"{"referenceValue":"ref"}"#)?;
        assert_eq!(deserialized, Value::Reference("ref".to_owned()));
        let deserialized: Value =
            serde_json::from_str(r#"{"geoPointValue":{"latitude":123.456,"longitude":789.012}}"#)?;
        assert_eq!(
            deserialized,
            Value::GeoPoint(LatLng {
                latitude: NotNan::new(123.456)?,
                longitude: NotNan::new(789.012)?,
            })
        );
        let deserialized: Value =
            serde_json::from_str(r#"{"arrayValue":{"values":[{"nullValue":null}]}}"#)?;
        assert_eq!(
            deserialized,
            Value::Array(Array {
                values: vec![Value::Null]
            })
        );
        let deserialized: Value =
            serde_json::from_str(r#"{"mapValue":{"fields":{"s":{"stringValue":"t"}}}}"#)?;
        assert_eq!(
            deserialized,
            Value::Map(Map {
                fields: {
                    let mut map = HashMap::new();
                    map.insert("s".to_owned(), Value::String("t".to_owned()));
                    map
                }
            })
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&Value::Null)?,
            r#"{"nullValue":null}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Boolean(true))?,
            r#"{"booleanValue":true}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Integer(123))?,
            r#"{"integerValue":"123"}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Double(NotNan::new(123.456)?))?,
            r#"{"doubleValue":123.456}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Timestamp("2001-02-03T04:05:06Z".to_owned()))?,
            r#"{"timestampValue":"2001-02-03T04:05:06Z"}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::String("s".to_owned()))?,
            r#"{"stringValue":"s"}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Bytes("Ynl0ZXMK".to_owned()))?,
            r#"{"bytesValue":"Ynl0ZXMK"}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Reference("ref".to_owned()))?,
            r#"{"referenceValue":"ref"}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::GeoPoint(LatLng {
                latitude: NotNan::new(123.456)?,
                longitude: NotNan::new(789.012)?,
            }))?,
            r#"{"geoPointValue":{"latitude":123.456,"longitude":789.012}}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Array(Array {
                values: vec![Value::Null]
            }))?,
            r#"{"arrayValue":{"values":[{"nullValue":null}]}}"#
        );
        assert_eq!(
            serde_json::to_string(&Value::Map(Map {
                fields: {
                    let mut map = HashMap::new();
                    map.insert("s".to_owned(), Value::String("t".to_owned()));
                    map
                }
            }))?,
            r#"{"mapValue":{"fields":{"s":{"stringValue":"t"}}}}"#
        );
        Ok(())
    }
}
