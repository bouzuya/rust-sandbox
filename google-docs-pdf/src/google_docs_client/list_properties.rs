use super::nesting_level::NestingLevel;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#listproperties>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_levels: Option<Vec<NestingLevel>>,
}
