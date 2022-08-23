use super::Timestamp;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Precondition {
    Exists(bool),
    UpdateTime(Timestamp),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: Precondition = serde_json::from_str(r#"{"exists":true}"#)?;
        assert_eq!(deserialized, Precondition::Exists(true));
        let deserialized: Precondition =
            serde_json::from_str(r#"{"updateTime":"2001-02-03T04:05:06Z"}"#)?;
        assert_eq!(
            deserialized,
            Precondition::UpdateTime("2001-02-03T04:05:06Z".to_owned())
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&Precondition::Exists(true))?,
            r#"{"exists":true}"#
        );
        assert_eq!(
            serde_json::to_string(&Precondition::UpdateTime("2001-02-03T04:05:06Z".to_owned()))?,
            r#"{"updateTime":"2001-02-03T04:05:06Z"}"#
        );
        Ok(())
    }
}
