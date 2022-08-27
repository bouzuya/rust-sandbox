use super::{StructuredQuery, Timestamp, TransactionOptions};

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunQueryRequestBody {
    pub structured_query: StructuredQuery,
    // consistency_selector: transaction or new_transaction or read_time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_transaction: Option<TransactionOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_time: Option<Timestamp>,
}

#[cfg(test)]
mod tests {
    use crate::firestore_rest::{
        structured_query::StructuredQuery, tests::serde_test, CollectionSelector, Cursor,
        Direction, FieldFilter, FieldOperator, FieldReference, Filter, Order, Projection, Value,
    };

    use super::*;

    #[test]
    fn serde_tests() -> anyhow::Result<()> {
        serde_test(
            RunQueryRequestBody {
                structured_query: StructuredQuery {
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
                transaction: None,
                new_transaction: None,
                read_time: None,
            },
            r#"{"structuredQuery":{"select":{"fields":[{"fieldPath":"id"}]},"from":[{"collectionId":"events","allDescendants":false}],"where":{"fieldFilter":{"field":{"fieldPath":"stream_id"},"op":"EQUAL","value":{"stringValue":"2a24ebc5-fc1d-4282-8b9c-2b295905b605"}}},"orderBy":[{"field":{"fieldPath":"stream_seq"},"direction":"ASCENDING"}],"startAt":{"values":[{"stringValue":"start"}],"before":false},"endAt":{"values":[{"stringValue":"end"}],"before":false},"offset":0,"limit":100}}"#,
        )?;
        Ok(())
    }
}
