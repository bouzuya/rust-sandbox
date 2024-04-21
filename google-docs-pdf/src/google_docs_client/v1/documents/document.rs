use std::collections::HashMap;

use crate::google_docs_client::v1::documents::{
    Body, DocumentStyle, Footer, Footnote, Header, InlineObject, List, NamedRanges, NamedStyles,
    PositionedObject, SuggestedDocumentStyle, SuggestedNamedStyles, SuggestionsViewMode,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#resource:-document>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Header>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footers: Option<HashMap<String, Footer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footnotes: Option<HashMap<String, Footnote>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_style: Option<DocumentStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_document_style_changes: Option<HashMap<String, SuggestedDocumentStyle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_styles: Option<NamedStyles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_named_styles_changes: Option<HashMap<String, SuggestedNamedStyles>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<HashMap<String, List>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_ranges: Option<HashMap<String, NamedRanges>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions_view_mode: Option<SuggestionsViewMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_objects: Option<HashMap<String, InlineObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positioned_objects: Option<HashMap<String, PositionedObject>>,
}
