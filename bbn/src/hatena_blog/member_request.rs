use crate::{
    data::Timestamp,
    hatena_blog::{HatenaBlogEntryId, MemberRequestId},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberRequest {
    pub id: MemberRequestId,
    pub at: Timestamp,
    pub hatena_blog_entry_id: HatenaBlogEntryId,
}
