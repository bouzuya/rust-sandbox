/// <https://developers.google.com/docs/api/reference/rest/v1/documents#sheetschartreferencesuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetsChartReferenceSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_id_suggested: Option<bool>,
}
