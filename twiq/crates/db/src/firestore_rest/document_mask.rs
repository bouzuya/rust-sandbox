#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMask {
    pub field_paths: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: DocumentMask = serde_json::from_str(r#"{"fieldPaths":["p"]}"#)?;
        assert_eq!(
            deserialized,
            DocumentMask {
                field_paths: vec!["p".to_owned()]
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&DocumentMask {
                field_paths: vec!["p".to_owned()]
            })?,
            r#"{"fieldPaths":["p"]}"#
        );
        Ok(())
    }
}
