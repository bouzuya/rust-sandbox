#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FieldOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    ArrayContains,
    In,
    ArrayContainsAny,
    NotIn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: FieldOperator = serde_json::from_str(r#""LESS_THAN""#)?;
        assert_eq!(deserialized, FieldOperator::LessThan);
        let deserialized: FieldOperator = serde_json::from_str(r#""LESS_THAN_OR_EQUAL""#)?;
        assert_eq!(deserialized, FieldOperator::LessThanOrEqual);
        let deserialized: FieldOperator = serde_json::from_str(r#""GREATER_THAN""#)?;
        assert_eq!(deserialized, FieldOperator::GreaterThan);
        let deserialized: FieldOperator = serde_json::from_str(r#""GREATER_THAN_OR_EQUAL""#)?;
        assert_eq!(deserialized, FieldOperator::GreaterThanOrEqual);
        let deserialized: FieldOperator = serde_json::from_str(r#""EQUAL""#)?;
        assert_eq!(deserialized, FieldOperator::Equal);
        let deserialized: FieldOperator = serde_json::from_str(r#""NOT_EQUAL""#)?;
        assert_eq!(deserialized, FieldOperator::NotEqual);
        let deserialized: FieldOperator = serde_json::from_str(r#""ARRAY_CONTAINS""#)?;
        assert_eq!(deserialized, FieldOperator::ArrayContains);
        let deserialized: FieldOperator = serde_json::from_str(r#""IN""#)?;
        assert_eq!(deserialized, FieldOperator::In);
        let deserialized: FieldOperator = serde_json::from_str(r#""ARRAY_CONTAINS_ANY""#)?;
        assert_eq!(deserialized, FieldOperator::ArrayContainsAny);
        let deserialized: FieldOperator = serde_json::from_str(r#""NOT_IN""#)?;
        assert_eq!(deserialized, FieldOperator::NotIn);
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&FieldOperator::LessThan)?,
            r#""LESS_THAN""#
        );
        assert_eq!(
            serde_json::to_string(&FieldOperator::LessThanOrEqual)?,
            r#""LESS_THAN_OR_EQUAL""#
        );
        assert_eq!(
            serde_json::to_string(&FieldOperator::GreaterThan)?,
            r#""GREATER_THAN""#
        );
        assert_eq!(
            serde_json::to_string(&FieldOperator::GreaterThanOrEqual)?,
            r#""GREATER_THAN_OR_EQUAL""#
        );
        assert_eq!(serde_json::to_string(&FieldOperator::Equal)?, r#""EQUAL""#);
        assert_eq!(
            serde_json::to_string(&FieldOperator::NotEqual)?,
            r#""NOT_EQUAL""#
        );
        assert_eq!(
            serde_json::to_string(&FieldOperator::ArrayContains)?,
            r#""ARRAY_CONTAINS""#
        );
        assert_eq!(serde_json::to_string(&FieldOperator::In)?, r#""IN""#);
        assert_eq!(
            serde_json::to_string(&FieldOperator::ArrayContainsAny)?,
            r#""ARRAY_CONTAINS_ANY""#
        );
        assert_eq!(serde_json::to_string(&FieldOperator::NotIn)?, r#""NOT_IN""#);
        Ok(())
    }
}
