use super::Value;

// <https://cloud.google.com/firestore/docs/reference/rest/Shared.Types/ArrayValue>
#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ArrayValue {
    pub values: Vec<Value>,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            ArrayValue {
                values: vec![Value::Null, Value::Boolean(true)],
            },
            r#"{"values":[{"nullValue":null},{"booleanValue":true}]}"#,
        )?;
        Ok(())
    }
}
