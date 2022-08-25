use super::{FieldReference, UnaryOperator};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UnaryFilter {
    pub op: UnaryOperator,
    pub field: FieldReference,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: UnaryFilter =
            serde_json::from_str(r#"{"op":"IS_NAN","field":{"fieldPath":"f"}}"#)?;
        assert_eq!(
            deserialized,
            UnaryFilter {
                op: UnaryOperator::IsNan,
                field: FieldReference {
                    field_path: "f".to_owned()
                }
            }
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&UnaryFilter {
                op: UnaryOperator::IsNan,
                field: FieldReference {
                    field_path: "f".to_owned()
                }
            })?,
            r#"{"op":"IS_NAN","field":{"fieldPath":"f"}}"#
        );
        Ok(())
    }
}
