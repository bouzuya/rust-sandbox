#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldReference {
    pub field_path: String,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            FieldReference {
                field_path: "max(messages.time) as max_time".to_owned(),
            },
            r#"{"fieldPath":"max(messages.time) as max_time"}"#,
        )?;
        Ok(())
    }
}
