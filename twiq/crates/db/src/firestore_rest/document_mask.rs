#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMask {
    pub field_paths: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            DocumentMask {
                field_paths: vec!["p".to_owned()],
            },
            r#"{"fieldPaths":["p"]}"#,
        )?;
        Ok(())
    }
}
