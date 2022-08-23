use super::TransactionOptions;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BeginTransactionRequestBody {
    pub options: TransactionOptions,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: BeginTransactionRequestBody =
            serde_json::from_str(r#"{"options":{"readWrite":{}}}"#)?;
        assert_eq!(
            deserialized,
            BeginTransactionRequestBody {
                options: TransactionOptions::ReadWrite {
                    retry_transaction: None,
                },
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&BeginTransactionRequestBody {
                options: TransactionOptions::ReadWrite {
                    retry_transaction: None,
                },
            })?,
            r#"{"options":{"readWrite":{}}}"#
        );
        assert_eq!(
            serde_json::to_string(&BeginTransactionRequestBody {
                options: TransactionOptions::ReadWrite {
                    retry_transaction: Some("abc".to_owned()),
                },
            })?,
            r#"{"options":{"readWrite":{"retryTransaction":"abc"}}}"#
        );
        assert_eq!(
            serde_json::to_string(&BeginTransactionRequestBody {
                options: TransactionOptions::ReadOnly {
                    read_time: "2000-01-02T03:04:05Z".to_owned()
                }
            })?,
            r#"{"options":{"readOnly":{"readTime":"2000-01-02T03:04:05Z"}}}"#
        );
        Ok(())
    }
}
