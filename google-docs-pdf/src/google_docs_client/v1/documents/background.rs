use crate::google_docs_client::v1::documents::OptionalColor;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#background>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<OptionalColor>,
}
