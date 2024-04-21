use crate::google_docs_client::v1::documents::SheetsChartReferenceSuggestionState;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#linkedcontentreferencesuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedContentReferenceSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets_chart_reference_suggestion_state: Option<SheetsChartReferenceSuggestionState>,
}
