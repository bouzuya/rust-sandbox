use std::collections::HashMap;

use crate::google_docs_client::v1::documents::{SuggestedTextStyle, TextStyle};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#columnbreak>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnBreak {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_text_style_changes: Option<HashMap<String, SuggestedTextStyle>>,
}
