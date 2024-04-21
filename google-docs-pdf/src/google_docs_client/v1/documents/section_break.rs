use crate::google_docs_client::v1::documents::SectionStyle;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#sectionbreak>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionBreak {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_insertion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_deletion_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_style: Option<SectionStyle>,
}
