use super::{FieldReference, UnaryOperator};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UnaryFilter {
    pub op: UnaryOperator,
    pub field: FieldReference,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::tests::serde_test;

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            UnaryFilter {
                op: UnaryOperator::IsNan,
                field: FieldReference {
                    field_path: "f".to_owned(),
                },
            },
            r#"{"op":"IS_NAN","field":{"fieldPath":"f"}}"#,
        )?;
        Ok(())
    }
}
