use crate::{session_id::SessionId, user_id::UserId};

#[derive(Clone, Eq, PartialEq)]
pub struct Session {
    pub(crate) id: SessionId,
    pub(crate) nonce: Option<String>,
    pub(crate) state: Option<String>,
    pub(crate) user_id: UserId,
}
