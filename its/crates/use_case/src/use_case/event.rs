use domain::{IssueCreatedV2, IssueFinished, IssueUpdated};

#[derive(Debug)]
pub enum IssueManagementContextEvent {
    IssueCreated(IssueCreatedV2),
    IssueFinished(IssueFinished),
    IssueUpdated(IssueUpdated),
}
