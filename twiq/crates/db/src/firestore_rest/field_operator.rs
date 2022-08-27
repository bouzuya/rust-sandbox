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
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        use FieldOperator::*;
        serde_test(LessThan, r#""LESS_THAN""#)?;
        serde_test(LessThanOrEqual, r#""LESS_THAN_OR_EQUAL""#)?;
        serde_test(GreaterThan, r#""GREATER_THAN""#)?;
        serde_test(GreaterThanOrEqual, r#""GREATER_THAN_OR_EQUAL""#)?;
        serde_test(Equal, r#""EQUAL""#)?;
        serde_test(NotEqual, r#""NOT_EQUAL""#)?;
        serde_test(ArrayContains, r#""ARRAY_CONTAINS""#)?;
        serde_test(In, r#""IN""#)?;
        serde_test(ArrayContainsAny, r#""ARRAY_CONTAINS_ANY""#)?;
        serde_test(NotIn, r#""NOT_IN""#)?;
        Ok(())
    }
}
