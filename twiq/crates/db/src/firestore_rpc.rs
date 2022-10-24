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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(
                value_from_string("abc".to_owned()),
                Value {
                    value_type: Some(ValueType::StringValue("abc".to_owned()))
                }
            );
        }
    }
}
