/// <https://developers.google.com/docs/api/reference/rest/v1/documents#embeddedobjectbordersuggestionstate>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedObjectBorderSuggestionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_style_suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_state_suggested: Option<bool>,
}
