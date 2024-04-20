use super::dimension::Dimension;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#sectioncolumnproperties>
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionColumnProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Dimension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_end: Option<Dimension>,
}
