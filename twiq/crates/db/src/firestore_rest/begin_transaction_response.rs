#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BeginTransactionResponse {
    pub transaction: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: BeginTransactionResponse =
            serde_json::from_str(r#"{"transaction":"abc"}"#)?;
        assert_eq!(
            deserialized,
            BeginTransactionResponse {
                transaction: "abc".to_owned()
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&BeginTransactionResponse {
                transaction: "abc".to_owned()
            })?,
            r#"{"transaction":"abc"}"#
        );
        Ok(())
    }
}
