use super::{
    paragraph::Paragraph, section_break::SectionBreak, table::Table,
    table_of_contents::TableOfContents,
};

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
