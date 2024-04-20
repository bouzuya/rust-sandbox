/// <https://developers.google.com/docs/api/reference/rest/v1/documents#shadingsuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShadingSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color_suggested: Option<bool>,
}
