use super::Value;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Cursor {
    pub values: Vec<Value>,
    pub before: bool,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            Cursor {
                values: vec![Value::String("s".to_owned())],
                before: false,
            },
            r#"{"values":[{"stringValue":"s"}],"before":false}"#,
        )?;
        Ok(())
    }
}
