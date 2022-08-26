use super::{CompositeFilter, FieldFilter, UnaryFilter};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Filter {
    #[serde(rename = "compositeFilter")]
    Composite(CompositeFilter),
    #[serde(rename = "fieldFilter")]
    Field(FieldFilter),
    #[serde(rename = "unaryFilter")]
    Unary(UnaryFilter),
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::{
        CompositeOperator, FieldOperator, FieldReference, UnaryOperator, Value,
    };

    use super::*;

    #[test]
    fn deserialize_test() -> anyhow::Result<()> {
        let deserialized: Filter =
            serde_json::from_str(r#"{"unaryFilter":{"op":"IS_NAN","field":{"fieldPath":"f"}}}"#)?;
        assert_eq!(
            deserialized,
            Filter::Unary(UnaryFilter {
                op: UnaryOperator::IsNan,
                field: FieldReference {
                    field_path: "f".to_owned()
                }
            })
        );
        let deserialized: Filter = serde_json::from_str(
            r#"{"fieldFilter":{"field":{"fieldPath":"a"},"op":"EQUAL","value":{"stringValue":"b"}}}"#,
        )?;
        assert_eq!(
            deserialized,
            Filter::Field(FieldFilter {
                field: FieldReference {
                    field_path: "a".to_owned()
                },
                op: FieldOperator::Equal,
                value: Value::String("b".to_owned())
            })
        );
        let deserialized: Filter = serde_json::from_str(
            r#"{"compositeFilter":{"op":"AND","filters":[{"unaryFilter":{"op":"IS_NAN","field":{"fieldPath":"f"}}}]}}"#,
        )?;
        assert_eq!(
            deserialized,
            Filter::Composite(CompositeFilter {
                op: CompositeOperator::And,
                filters: vec![Filter::Unary(UnaryFilter {
                    op: UnaryOperator::IsNan,
                    field: FieldReference {
                        field_path: "f".to_owned()
                    }
                })]
            })
        );
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&Filter::Unary(UnaryFilter {
                op: UnaryOperator::IsNan,
                field: FieldReference {
                    field_path: "f".to_owned()
                }
            }))?,
            r#"{"unaryFilter":{"op":"IS_NAN","field":{"fieldPath":"f"}}}"#
        );
        assert_eq!(
            serde_json::to_string(&Filter::Field(FieldFilter {
                field: FieldReference {
                    field_path: "a".to_owned()
                },
                op: FieldOperator::Equal,
                value: Value::String("b".to_owned())
            }))?,
            r#"{"fieldFilter":{"field":{"fieldPath":"a"},"op":"EQUAL","value":{"stringValue":"b"}}}"#,
        );
        assert_eq!(
            serde_json::to_string(&Filter::Composite(CompositeFilter {
                op: CompositeOperator::And,
                filters: vec![Filter::Unary(UnaryFilter {
                    op: UnaryOperator::IsNan,
                    field: FieldReference {
                        field_path: "f".to_owned()
                    }
                })]
            }))?,
            r#"{"compositeFilter":{"op":"AND","filters":[{"unaryFilter":{"op":"IS_NAN","field":{"fieldPath":"f"}}}]}}"#,
        );
        Ok(())
    }
}
