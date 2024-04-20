use std::collections::HashMap;

use super::{
    inline_object_properties::InlineObjectProperties,
    suggested_inline_object_properties::SuggestedInlineObjectProperties,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#inlineobject>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_object_properties: Option<InlineObjectProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_inline_object_properties_changes:
        Option<HashMap<String, SuggestedInlineObjectProperties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
}
