use super::TransactionOptions;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BeginTransactionRequestBody {
    pub options: TransactionOptions,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            BeginTransactionRequestBody {
                options: TransactionOptions::ReadWrite {
                    retry_transaction: None,
                },
            },
            r#"{"options":{"readWrite":{}}}"#,
        )?;
        serde_test(
            BeginTransactionRequestBody {
                options: TransactionOptions::ReadWrite {
                    retry_transaction: Some("abc".to_owned()),
                },
            },
            r#"{"options":{"readWrite":{"retryTransaction":"abc"}}}"#,
        )?;
        serde_test(
            BeginTransactionRequestBody {
                options: TransactionOptions::ReadOnly {
                    read_time: "2000-01-02T03:04:05Z".to_owned(),
                },
            },
            r#"{"options":{"readOnly":{"readTime":"2000-01-02T03:04:05Z"}}}"#,
        )?;
        Ok(())
    }
}
