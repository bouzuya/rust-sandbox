use crate::google_docs_client::v1::documents::{Paragraph, SectionBreak, Table, TableOfContents};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#structuralelement>
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StructuralElementContent {
    Paragraph(Paragraph),
    SectionBreak(SectionBreak),
    Table(Table),
    TableOfContents(TableOfContents),
}
