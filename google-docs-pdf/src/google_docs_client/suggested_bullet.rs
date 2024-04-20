use super::{bullet::Bullet, bullet_suggestion_state::BulletSuggestionState};

/// <https://developers.google.com/docs/api/reference/rest/v1/documents#suggestedbullet>
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedBullet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet: Option<Bullet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bullet_suggestion_state: Option<BulletSuggestionState>,
}
