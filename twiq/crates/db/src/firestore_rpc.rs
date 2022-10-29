pub mod google {
    pub mod api {
        include!("firestore_rpc/google.api.rs");
    }
    pub mod firestore {
        pub mod v1 {
            include!("firestore_rpc/google.firestore.v1.rs");
        }
    }
    pub mod protobuf {
        include!("firestore_rpc/google.protobuf.rs");
    }
    pub mod r#type {
        include!("firestore_rpc/google.r#type.rs");
    }
    pub mod rpc {
        include!("firestore_rpc/google.rpc.rs");
    }
}

pub mod helper {
    use prost_types::Timestamp;

    use super::google::firestore::v1::{value::ValueType, Document, Value};

    pub fn get_field_as_i64(document: &Document, key: &str) -> Option<i64> {
        document.fields.get(key).map(value_into_i64_unchecked)
    }

    pub fn get_field_as_str<'a>(document: &'a Document, key: &'a str) -> Option<&'a str> {
        document.fields.get(key).map(value_as_str_unchecked)
    }

    pub fn get_field_as_timestamp(document: &Document, key: &str) -> Option<Timestamp> {
        document.fields.get(key).map(value_to_timestamp_unchecked)
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
        fn get_field_as_i64_test() {
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
                ),
                Some(123)
            );
        }

        #[test]
        fn get_field_as_str_test() {
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
                ),
                Some("val")
            );
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
                ),
                Some(timestamp)
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
