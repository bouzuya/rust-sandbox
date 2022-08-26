use super::{Direction, FieldReference};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub field: FieldReference,
    pub direction: Direction,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            Order {
                field: FieldReference {
                    field_path: "f".to_owned(),
                },
                direction: Direction::Ascending,
            },
            r#"{"field":{"fieldPath":"f"},"direction":"ASCENDING"}"#,
        )?;
        Ok(())
    }
}
