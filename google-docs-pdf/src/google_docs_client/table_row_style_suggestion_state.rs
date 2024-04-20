/// <https://developers.google.com/docs/api/reference/rest/v1/documents#tablerowstylesuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowStyleSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_row_height_suggested: Option<bool>,
}
