/// <https://developers.google.com/docs/api/reference/rest/v1/documents#positionedobjectlayout>
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionedObjectLayout {
    #[allow(clippy::enum_variant_names)]
    #[default]
    PositionedObjectLayoutUnspecified,
    WrapText,
    BreakLeft,
    BreakRight,
    BreakLeftRight,
    InFrontOfText,
    BehindText,
}

#[cfg(test)]
mod tests {
    use crate::google_docs_client::tests::test_serde;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        for (s, v) in [
            (
                r#""POSITIONED_OBJECT_LAYOUT_UNSPECIFIED""#,
                PositionedObjectLayout::PositionedObjectLayoutUnspecified,
            ),
            (r#""WRAP_TEXT""#, PositionedObjectLayout::WrapText),
            (r#""BREAK_LEFT""#, PositionedObjectLayout::BreakLeft),
            (r#""BREAK_RIGHT""#, PositionedObjectLayout::BreakRight),
            (
                r#""BREAK_LEFT_RIGHT""#,
                PositionedObjectLayout::BreakLeftRight,
            ),
            (
                r#""IN_FRONT_OF_TEXT""#,
                PositionedObjectLayout::InFrontOfText,
            ),
            (r#""BEHIND_TEXT""#, PositionedObjectLayout::BehindText),
        ] {
            test_serde(s, v)?;
        }
        Ok(())
    }
}
