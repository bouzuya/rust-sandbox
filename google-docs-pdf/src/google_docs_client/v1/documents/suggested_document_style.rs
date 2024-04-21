use crate::google_docs_client::v1::documents::{DocumentStyle, DocumentStyleSuggestionState};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggesteddocumentstyle>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedDocumentStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_style: Option<DocumentStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_style_suggestion_state: Option<DocumentStyleSuggestionState>,
}
