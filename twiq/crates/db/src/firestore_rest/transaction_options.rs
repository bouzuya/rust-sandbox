#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionOptions {
    #[serde(rename_all = "camelCase")]
    ReadOnly { read_time: String },
    #[serde(rename_all = "camelCase")]
    ReadWrite {
        #[serde(skip_serializing_if = "Option::is_none")]
        retry_transaction: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: TransactionOptions =
            serde_json::from_str(r#"{"readOnly":{"readTime":"123"}}"#)?;
        assert_eq!(
            deserialized,
            TransactionOptions::ReadOnly {
                read_time: "123".to_owned()
            },
        );
        let deserialized: TransactionOptions =
            serde_json::from_str(r#"{"readWrite":{"retryTransaction":"123"}}"#)?;
        assert_eq!(
            deserialized,
            TransactionOptions::ReadWrite {
                retry_transaction: Some("123".to_owned())
            },
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&TransactionOptions::ReadOnly {
                read_time: "123".to_owned()
            })?,
            r#"{"readOnly":{"readTime":"123"}}"#
        );
        assert_eq!(
            serde_json::to_string(&TransactionOptions::ReadWrite {
                retry_transaction: Some("123".to_owned())
            })?,
            r#"{"readWrite":{"retryTransaction":"123"}}"#
        );
        Ok(())
    }
}
