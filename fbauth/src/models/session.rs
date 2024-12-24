use crate::models::{session_id::SessionId, user_id::UserId};

#[derive(Clone, Eq, PartialEq)]
pub struct Session {
    pub(crate) id: SessionId,
    pub(crate) nonce: Option<String>,
    pub(crate) state: Option<String>,
    pub(crate) user_id: Option<UserId>,
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("id", &self.id)
            .field("nonce", &"[FILTERED]")
            .field("state", &"[FILTERED]")
            .field("user_id", &self.user_id)
            .finish()
    }
}
