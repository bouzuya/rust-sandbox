#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    Ascending,
    Descending,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(Direction::Ascending, r#""ASCENDING""#)?;
        serde_test(Direction::Descending, r#""DESCENDING""#)?;
        Ok(())
    }
}
