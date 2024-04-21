/// <https://developers.google.com/docs/api/reference/rest/v1/documents#equation>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Equation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
}
