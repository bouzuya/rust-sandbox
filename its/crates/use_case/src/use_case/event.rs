use domain::{IssueBlockLinkId, IssueCommentId, IssueId};

#[derive(Clone, Debug)]
pub enum IssueManagementContextEvent {
    IssueCommentCreated {
        issue_comment_id: IssueCommentId,
    },
    IssueCreated {
        issue_id: IssueId,
    },
    IssueUpdated {
        issue_id: IssueId,
    },
    IssueBlocked {
        issue_block_link_id: IssueBlockLinkId,
    },
    IssueUnblocked {
        issue_block_link_id: IssueBlockLinkId,
    },
}
