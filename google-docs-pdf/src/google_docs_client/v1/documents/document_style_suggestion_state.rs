use crate::google_docs_client::v1::documents::{BackgroundSuggestionState, SizeSuggestionState};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#documentstylesuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentStyleSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_suggestion_state: Option<BackgroundSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_header_id_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_footer_id_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_page_header_id_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_page_footer_id_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page_header_id_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page_footer_id_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_first_page_header_footer_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_even_page_header_footer_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number_start_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size_suggestion_state: Option<SizeSuggestionState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_header_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_footer_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_custom_header_footer_margins_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flip_page_orientation_suggested: Option<bool>,
}
