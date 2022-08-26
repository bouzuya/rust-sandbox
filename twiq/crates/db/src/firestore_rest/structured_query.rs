use super::{CollectionSelector, Cursor, Filter, Order, Projection};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuredQuery {
    pub select: Projection,
    pub from: Vec<CollectionSelector>,
    pub r#where: Filter,
    pub order_by: Vec<Order>,
    pub start_at: Option<Cursor>,
    pub end_at: Option<Cursor>,
    pub offset: i32, // i32 (>= 0)
    pub limit: i32,  // i32 (>= 0)
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::{
        tests::serde_test, Direction, FieldFilter, FieldOperator, FieldReference, Value,
    };

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            StructuredQuery {
                select: Projection {
                    fields: vec![FieldReference {
                        field_path: "id".to_owned(),
                    }],
                },
                from: vec![CollectionSelector {
                    collection_id: "events".to_owned(),
                    all_descendants: false,
                }],
                r#where: Filter::Field(FieldFilter {
                    field: FieldReference {
                        field_path: "stream_id".to_owned(),
                    },
                    op: FieldOperator::Equal,
                    value: Value::String("2a24ebc5-fc1d-4282-8b9c-2b295905b605".to_owned()),
                }),
                order_by: vec![Order {
                    field: FieldReference {
                        field_path: "stream_seq".to_owned(),
                    },
                    direction: Direction::Ascending,
                }],
                start_at: Some(Cursor {
                    values: vec![Value::String("start".to_owned())],
                    before: false,
                }),
                end_at: Some(Cursor {
                    values: vec![Value::String("end".to_owned())],
                    before: false,
                }),
                offset: 0,
                limit: 100,
            },
            r#"{"select":{"fields":[{"fieldPath":"id"}]},"from":[{"collectionId":"events","allDescendants":false}],"where":{"fieldFilter":{"field":{"fieldPath":"stream_id"},"op":"EQUAL","value":{"stringValue":"2a24ebc5-fc1d-4282-8b9c-2b295905b605"}}},"orderBy":[{"field":{"fieldPath":"stream_seq"},"direction":"ASCENDING"}],"startAt":{"values":[{"stringValue":"start"}],"before":false},"endAt":{"values":[{"stringValue":"end"}],"before":false},"offset":0,"limit":100}"#,
        )?;
        Ok(())
    }
}
