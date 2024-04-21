use crate::google_docs_client::v1::documents::StructuralElementContent;

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#structuralelement>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<usize>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub content: Option<StructuralElementContent>,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::{
        tests::test_serde,
        v1::documents::{
            ColumnSeparatorStyle, ContentDirection, NamedStyleType, Paragraph, ParagraphElement,
            ParagraphElementContent, ParagraphStyle, SectionBreak, SectionStyle, SectionType,
            TextRun,
        },
    };

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#"
            {
              "startIndex": 1,
              "endIndex": 6,
              "paragraph": {
                "elements": [
                  {
                    "startIndex": 1,
                    "endIndex": 6,
                    "textRun": {
                      "content": "レポート\n",
                      "textStyle": {}
                    }
                  }
                ],
                "paragraphStyle": {
                  "headingId": "h.9hwcq7fu71a1",
                  "namedStyleType": "HEADING_2",
                  "direction": "LEFT_TO_RIGHT"
                }
              }
            }
            "#,
                StructuralElement {
                    start_index: Some(1),
                    end_index: Some(6),
                    content: Some(StructuralElementContent::Paragraph(Paragraph {
                        elements: Some(vec![ParagraphElement {
                            start_index: Some(1),
                            end_index: Some(6),
                            content: Some(ParagraphElementContent::TextRun(TextRun {
                                content: Some("レポート\n".to_string()),
                                text_style: Some(Default::default()),
                                ..Default::default()
                            })),
                        }]),
                        paragraph_style: Some(ParagraphStyle {
                            heading_id: Some("h.9hwcq7fu71a1".to_string()),
                            named_style_type: Some(NamedStyleType::Heading_2),
                            direction: Some(ContentDirection::LeftToRight),
                            ..Default::default()
                        }),
                        ..Default::default()
                    })),
                },
            ),
            (
                r#"
{
  "endIndex": 1,
  "sectionBreak": {
    "sectionStyle": {
      "columnSeparatorStyle": "NONE",
      "contentDirection": "LEFT_TO_RIGHT",
      "sectionType": "CONTINUOUS"
    }
  }
}
"#,
                StructuralElement {
                    start_index: None,
                    end_index: Some(1),
                    content: Some(StructuralElementContent::SectionBreak(SectionBreak {
                        suggested_insertion_ids: None,
                        suggested_deletion_ids: None,
                        section_style: Some(SectionStyle {
                            column_separator_style: Some(ColumnSeparatorStyle::None),
                            content_direction: Some(ContentDirection::LeftToRight),
                            section_type: Some(SectionType::Continuous),
                            ..Default::default()
                        }),
                    })),
                },
            ),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
