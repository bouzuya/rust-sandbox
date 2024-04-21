/// <https://developers.google.com/docs/api/reference/rest/v1/documents#objectreferences>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectReferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_ids: Option<Vec<String>>,
}
