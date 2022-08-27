use super::Write;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CommitRequestBody {
    pub writes: Vec<Write>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            CommitRequestBody {
                writes: vec![Write::Delete {
                    current_document: None,
                    delete: "123".to_owned(),
                }],
                transaction: None,
            },
            r#"{"writes":[{"delete":"123"}]}"#,
        )?;
        serde_test(
            CommitRequestBody {
                writes: vec![Write::Delete {
                    current_document: None,
                    delete: "123".to_owned(),
                }],
                transaction: Some("456".to_owned()),
            },
            r#"{"writes":[{"delete":"123"}],"transaction":"456"}"#,
        )?;
        Ok(())
    }
}
