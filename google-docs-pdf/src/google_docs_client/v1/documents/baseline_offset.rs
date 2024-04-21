/// <https://developers.google.com/docs/api/reference/rest/v1/documents#baselineoffset>
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
pub enum BaselineOffset {
    #[allow(clippy::enum_variant_names)]
    #[default]
    BaselineOffsetUnspecified,
    None,
    Superscript,
    Subscript,
}
