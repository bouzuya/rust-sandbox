use super::Value;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Array {
    pub values: Vec<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let array: Array =
            serde_json::from_str(r#"{"values":[{"nullValue":null},{"booleanValue":true}]}"#)?;
        assert_eq!(
            array,
            Array {
                values: vec![Value::Null, Value::Boolean(true)],
            },
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&Array {
                values: vec![Value::Null, Value::Boolean(true)],
            })?,
            r#"{"values":[{"nullValue":null},{"booleanValue":true}]}"#
        );
        Ok(())
    }
}
