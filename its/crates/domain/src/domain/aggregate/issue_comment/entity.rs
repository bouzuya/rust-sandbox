use crate::{IssueCommentId, IssueId};

use super::attribute::IssueCommentText;
use super::event::IssueCommentCreated;

#[derive(Debug)]
pub struct IssueComment {
    id: IssueCommentId,
    issue_id: IssueId,
    text: IssueCommentText,
}

impl IssueComment {
    pub fn from_event(event: IssueCommentCreated) -> Self {
        Self {
            id: event.id,
            issue_id: event.issue_id,
            text: event.text,
        }
    }

    pub fn new(id: IssueCommentId, issue_id: IssueId, text: IssueCommentText) -> Self {
        Self { id, issue_id, text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // TODO
    }
}
