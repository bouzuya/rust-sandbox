use super::{CompositeOperator, Filter};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CompositeFilter {
    pub op: CompositeOperator,
    pub filters: Vec<Filter>,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::{FieldReference, UnaryFilter, UnaryOperator};

    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: CompositeFilter = serde_json::from_str(
            r#"{"op":"AND","filters":[{"unaryFilter":{"op":"IS_NAN","field":{"fieldPath":"f"}}}]}"#,
        )?;
        assert_eq!(
            deserialized,
            CompositeFilter {
                op: CompositeOperator::And,
                filters: vec![Filter::Unary(UnaryFilter {
                    op: UnaryOperator::IsNan,
                    field: FieldReference {
                        field_path: "f".to_owned()
                    }
                })]
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&CompositeFilter {
                op: CompositeOperator::And,
                filters: vec![Filter::Unary(UnaryFilter {
                    op: UnaryOperator::IsNan,
                    field: FieldReference {
                        field_path: "f".to_owned()
                    }
                })]
            })?,
            r#"{"op":"AND","filters":[{"unaryFilter":{"op":"IS_NAN","field":{"fieldPath":"f"}}}]}"#,
        );
        Ok(())
    }
}
