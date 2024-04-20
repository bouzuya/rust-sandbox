use std::collections::HashMap;

use super::{
    bullet::Bullet, object_references::ObjectReferences, paragraph_element::ParagraphElement,
    paragraph_style::ParagraphStyle, suggested_bullet::SuggestedBullet,
    suggested_paragraph_style::SuggestedParagraphStyle,
};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#paragraph>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Paragraph {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<ParagraphElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_style: Option<ParagraphStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_paragraph_style_changes: Option<HashMap<String, SuggestedParagraphStyle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet: Option<Bullet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_bullet_changes: Option<HashMap<String, SuggestedBullet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positioned_object_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_positioned_object_ids: Option<HashMap<String, ObjectReferences>>,
}
