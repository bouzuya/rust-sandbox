use std::collections::HashMap;

use super::{r#type::Type, suggested_text_style::SuggestedTextStyle, text_style::TextStyle};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#autotext>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
}
