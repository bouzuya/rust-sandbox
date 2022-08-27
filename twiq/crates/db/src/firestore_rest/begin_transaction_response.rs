#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BeginTransactionResponse {
    pub transaction: String,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            BeginTransactionResponse {
                transaction: "abc".to_owned(),
            },
            r#"{"transaction":"abc"}"#,
        )?;
        Ok(())
    }
}
