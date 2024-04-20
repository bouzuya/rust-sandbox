use super::{
    auto_text::AutoText, column_break::ColumnBreak, equation::Equation,
    footnote_reference::FootnoteReference, horizontal_rule::HorizontalRule,
    inline_object_element::InlineObjectElement, page_break::PageBreak, person::Person,
    rich_link::RichLink, text_run::TextRun,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#paragraphelement>
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ParagraphElementContent {
    TextRun(TextRun),
    AutoText(AutoText),
    PageBreak(PageBreak),
    ColumnBreak(ColumnBreak),
    FootnoteReference(FootnoteReference),
    HorizontalRule(HorizontalRule),
    Equation(Equation),
    InlineObjectElement(InlineObjectElement),
    Person(Person),
    RichLink(RichLink),
}
