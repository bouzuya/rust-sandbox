use std::collections::HashMap;

use super::{Timestamp, Value};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub name: String,
    pub fields: HashMap<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<Timestamp>,
}

#[cfg(test)]
mod tests {
    use ordered_float::NotNan;

    use crate::firestore_rest::{ArrayValue, LatLng, MapValue, Value};

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
                    map.insert("array".to_owned(), Value::Array(ArrayValue { values: vec![Value::String("s".to_owned())] }));
                    map.insert("map".to_owned(), Value::Map(MapValue { fields: {
                        let mut map = HashMap::new();
                        map.insert("s".to_owned(), Value::String("s".to_owned()));
                        map
                    } }));
                    map
                },
                create_time: Some("2022-08-19T22:53:42.480950Z".to_owned()),
                update_time: Some("2022-08-19T22:53:42.480950Z".to_owned())
            }
        );
        assert!(serde_json::to_string(&document).is_ok());
        Ok(())
    }
}
