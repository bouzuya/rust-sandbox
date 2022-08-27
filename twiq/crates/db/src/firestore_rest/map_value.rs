use std::collections::HashMap;

use super::Value;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MapValue {
    pub fields: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            MapValue {
                fields: {
                    let mut map = HashMap::new();
                    map.insert("key".to_owned(), Value::Null);
                    map
                },
            },
            r#"{"fields":{"key":{"nullValue":null}}}"#,
        )?;
        Ok(())
    }
}
