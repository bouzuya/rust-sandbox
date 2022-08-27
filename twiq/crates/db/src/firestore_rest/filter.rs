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
        tests::serde_test, CompositeOperator, FieldOperator, FieldReference, UnaryOperator, Value,
    };

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            Filter::Unary(UnaryFilter {
                op: UnaryOperator::IsNan,
                field: FieldReference {
                    field_path: "f".to_owned(),
                },
            }),
            r#"{"unaryFilter":{"op":"IS_NAN","field":{"fieldPath":"f"}}}"#,
        )?;
        serde_test(
            Filter::Field(FieldFilter {
                field: FieldReference {
                    field_path: "a".to_owned(),
                },
                op: FieldOperator::Equal,
                value: Value::String("b".to_owned()),
            }),
            r#"{"fieldFilter":{"field":{"fieldPath":"a"},"op":"EQUAL","value":{"stringValue":"b"}}}"#,
        )?;
        serde_test(
            Filter::Composite(CompositeFilter {
                op: CompositeOperator::And,
                filters: vec![Filter::Unary(UnaryFilter {
                    op: UnaryOperator::IsNan,
                    field: FieldReference {
                        field_path: "f".to_owned(),
                    },
                })],
            }),
            r#"{"compositeFilter":{"op":"AND","filters":[{"unaryFilter":{"op":"IS_NAN","field":{"fieldPath":"f"}}}]}}"#,
        )?;
        Ok(())
    }
}
