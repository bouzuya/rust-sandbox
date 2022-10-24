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

    pub fn value_from_string(s: String) -> Value {
        Value {
            value_type: Some(ValueType::StringValue(s)),
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
        fn value_from_string_test() {
            assert_eq!(
                value_from_string("abc".to_owned()),
                Value {
                    value_type: Some(ValueType::StringValue("abc".to_owned()))
                }
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
