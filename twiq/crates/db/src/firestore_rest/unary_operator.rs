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
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        use UnaryOperator::*;
        serde_test(IsNan, r#""IS_NAN""#)?;
        serde_test(IsNull, r#""IS_NULL""#)?;
        serde_test(IsNotNan, r#""IS_NOT_NAN""#)?;
        serde_test(IsNotNull, r#""IS_NOT_NULL""#)?;
        Ok(())
    }
}
