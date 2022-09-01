#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RollbackRequestBody {
    pub transaction: String,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            RollbackRequestBody {
                transaction: "abc".to_owned(),
            },
            r#"{"transaction":"abc"}"#,
        )?;
        Ok(())
    }
}
