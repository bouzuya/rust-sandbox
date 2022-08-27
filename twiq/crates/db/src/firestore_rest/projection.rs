use super::FieldReference;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Projection {
    pub fields: Vec<FieldReference>,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            Projection {
                fields: vec![FieldReference {
                    field_path: "max(messages.time) as max_time".to_owned(),
                }],
            },
            r#"{"fields":[{"fieldPath":"max(messages.time) as max_time"}]}"#,
        )?;
        Ok(())
    }
}
