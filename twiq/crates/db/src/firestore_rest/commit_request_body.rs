use super::Write;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CommitRequestBody {
    pub writes: Vec<Write>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: CommitRequestBody =
            serde_json::from_str(r#"{"writes":[{"delete":"123"}]}"#)?;
        assert_eq!(
            deserialized,
            CommitRequestBody {
                writes: vec![Write::Delete("123".to_owned())],
                transaction: None
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&CommitRequestBody {
                writes: vec![Write::Delete("123".to_owned())],
                transaction: None
            })?,
            r#"{"writes":[{"delete":"123"}]}"#
        );
        assert_eq!(
            serde_json::to_string(&CommitRequestBody {
                writes: vec![Write::Delete("123".to_owned())],
                transaction: Some("456".to_owned())
            })?,
            r#"{"writes":[{"delete":"123"}],"transaction":"456"}"#
        );
        Ok(())
    }
}
