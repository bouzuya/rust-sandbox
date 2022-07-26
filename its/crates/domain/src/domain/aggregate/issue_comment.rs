pub mod attribute;
pub mod entity;
pub mod event;

use crate::Version;

use self::entity::IssueComment;
pub use self::event::Event;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCommentAggregate {
    events: Vec<Event>,
    issue_comment: IssueComment,
    version: Version,
}
