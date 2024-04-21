use crate::google_docs_client::v1::documents::{
    AutoText, ColumnBreak, Equation, FootnoteReference, HorizontalRule, InlineObjectElement,
    PageBreak, Person, RichLink, TextRun,
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
