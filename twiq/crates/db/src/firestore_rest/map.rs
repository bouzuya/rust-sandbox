use std::collections::HashMap;

use super::Value;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Map {
    pub fields: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: Map = serde_json::from_str(r#"{"fields":{"key":{"nullValue":null}}}"#)?;
        assert_eq!(
            deserialized,
            Map {
                fields: {
                    let mut map = HashMap::new();
                    map.insert("key".to_owned(), Value::Null);
                    map
                }
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&Map {
                fields: {
                    let mut map = HashMap::new();
                    map.insert("key".to_owned(), Value::Null);
                    map
                }
            })?,
            r#"{"fields":{"key":{"nullValue":null}}}"#
        );
        Ok(())
    }
}
