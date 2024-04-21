use std::collections::HashMap;

use crate::google_docs_client::v1::documents::{
    PositionedObjectProperties, SuggestedPositionedObjectProperties,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#positionedobject>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionedObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positioned_object_properties: Option<PositionedObjectProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_positioned_object_properties_changes:
        Option<HashMap<String, SuggestedPositionedObjectProperties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
}
