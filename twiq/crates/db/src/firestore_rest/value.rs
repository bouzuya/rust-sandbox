use ordered_float::NotNan;
use serde::{de::Visitor, ser::SerializeMap};

use super::{ArrayValue, LatLng, MapValue, Timestamp};

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
    Array(ArrayValue),
    Map(MapValue),
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

    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        use Value::*;
        serde_test(Null, r#"{"nullValue":null}"#)?;
        serde_test(Boolean(true), r#"{"booleanValue":true}"#)?;
        serde_test(Integer(123), r#"{"integerValue":"123"}"#)?;
        serde_test(Double(NotNan::new(123.456)?), r#"{"doubleValue":123.456}"#)?;
        serde_test(
            Timestamp("2001-02-03T04:05:06Z".to_owned()),
            r#"{"timestampValue":"2001-02-03T04:05:06Z"}"#,
        )?;
        serde_test(String("s".to_owned()), r#"{"stringValue":"s"}"#)?;
        serde_test(Bytes("Ynl0ZXMK".to_owned()), r#"{"bytesValue":"Ynl0ZXMK"}"#)?;
        serde_test(Reference("ref".to_owned()), r#"{"referenceValue":"ref"}"#)?;
        serde_test(
            GeoPoint(LatLng {
                latitude: NotNan::new(123.456)?,
                longitude: NotNan::new(789.012)?,
            }),
            r#"{"geoPointValue":{"latitude":123.456,"longitude":789.012}}"#,
        )?;
        serde_test(
            Array(ArrayValue {
                values: vec![Value::Null],
            }),
            r#"{"arrayValue":{"values":[{"nullValue":null}]}}"#,
        )?;
        serde_test(
            Map(MapValue {
                fields: {
                    let mut map = HashMap::new();
                    map.insert("s".to_owned(), Value::String("t".to_owned()));
                    map
                },
            }),
            r#"{"mapValue":{"fields":{"s":{"stringValue":"t"}}}}"#,
        )?;
        Ok(())
    }
}
