use super::Timestamp;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Precondition {
    Exists(bool),
    UpdateTime(Timestamp),
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(Precondition::Exists(true), r#"{"exists":true}"#)?;
        serde_test(
            Precondition::UpdateTime("2001-02-03T04:05:06Z".to_owned()),
            r#"{"updateTime":"2001-02-03T04:05:06Z"}"#,
        )?;
        Ok(())
    }
}
