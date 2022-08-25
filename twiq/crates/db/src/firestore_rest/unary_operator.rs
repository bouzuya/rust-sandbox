#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnaryOperator {
    IsNan,
    IsNull,
    IsNotNan,
    IsNotNull,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: UnaryOperator = serde_json::from_str(r#""IS_NAN""#)?;
        assert_eq!(deserialized, UnaryOperator::IsNan);
        let deserialized: UnaryOperator = serde_json::from_str(r#""IS_NULL""#)?;
        assert_eq!(deserialized, UnaryOperator::IsNull);
        let deserialized: UnaryOperator = serde_json::from_str(r#""IS_NOT_NAN""#)?;
        assert_eq!(deserialized, UnaryOperator::IsNotNan);
        let deserialized: UnaryOperator = serde_json::from_str(r#""IS_NOT_NULL""#)?;
        assert_eq!(deserialized, UnaryOperator::IsNotNull);
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&UnaryOperator::IsNan)?, r#""IS_NAN""#);
        assert_eq!(
            serde_json::to_string(&UnaryOperator::IsNull)?,
            r#""IS_NULL""#
        );
        assert_eq!(
            serde_json::to_string(&UnaryOperator::IsNotNan)?,
            r#""IS_NOT_NAN""#
        );
        assert_eq!(
            serde_json::to_string(&UnaryOperator::IsNotNull)?,
            r#""IS_NOT_NULL""#
        );
        Ok(())
    }
}
