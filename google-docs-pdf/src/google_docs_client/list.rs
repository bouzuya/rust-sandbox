use std::collections::HashMap;

use super::{list_properties::ListProperties, suggested_list_properties::SuggestedListProperties};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#list>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_properties: Option<ListProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_list_properties_changes: Option<HashMap<String, SuggestedListProperties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
}
