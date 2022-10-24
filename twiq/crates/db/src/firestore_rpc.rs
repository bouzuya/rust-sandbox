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
    use super::google::firestore::v1::{value::ValueType, Value};

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

    // panic if value_type is not integer
    pub fn value_into_i64_unchecked(value: Value) -> i64 {
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

    #[cfg(test)]
    mod tests {
        use super::*;

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
        fn value_into_i64_unchecked_test() {
            assert_eq!(
                value_into_i64_unchecked(Value {
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
    }
}
