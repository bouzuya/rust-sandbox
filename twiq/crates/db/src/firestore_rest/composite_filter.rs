use super::{CompositeOperator, Filter};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CompositeFilter {
    pub op: CompositeOperator,
    pub filters: Vec<Filter>,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::{tests::serde_test, FieldReference, UnaryFilter, UnaryOperator};

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            CompositeFilter {
                op: CompositeOperator::And,
                filters: vec![Filter::Unary(UnaryFilter {
                    op: UnaryOperator::IsNan,
                    field: FieldReference {
                        field_path: "f".to_owned(),
                    },
                })],
            },
            r#"{"op":"AND","filters":[{"unaryFilter":{"op":"IS_NAN","field":{"fieldPath":"f"}}}]}"#,
        )?;
        Ok(())
    }
}
