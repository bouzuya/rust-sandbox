#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompositeOperator {
    And,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: CompositeOperator = serde_json::from_str(r#""AND""#)?;
        assert_eq!(deserialized, CompositeOperator::And);
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&CompositeOperator::And)?, r#""AND""#);
        Ok(())
    }
}
