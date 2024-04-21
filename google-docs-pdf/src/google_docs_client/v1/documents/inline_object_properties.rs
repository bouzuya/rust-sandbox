use crate::google_docs_client::v1::documents::EmbeddedObject;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#inlineobjectproperties>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineObjectProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_object: Option<EmbeddedObject>,
}
