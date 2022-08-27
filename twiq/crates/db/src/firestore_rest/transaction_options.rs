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
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            TransactionOptions::ReadOnly {
                read_time: "123".to_owned(),
            },
            r#"{"readOnly":{"readTime":"123"}}"#,
        )?;
        serde_test(
            TransactionOptions::ReadWrite {
                retry_transaction: Some("123".to_owned()),
            },
            r#"{"readWrite":{"retryTransaction":"123"}}"#,
        )?;
        Ok(())
    }
}
