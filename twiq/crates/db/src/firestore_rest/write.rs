#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Write {
    // TODO:
    // Update {}
    Delete(String),
    // TODO:
    // Transform {},
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: Write = serde_json::from_str(r#"{"delete":"123"}"#)?;
        assert_eq!(deserialized, Write::Delete("123".to_owned()));
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&Write::Delete("123".to_owned()))?,
            r#"{"delete":"123"}"#
        );
        Ok(())
    }
}
