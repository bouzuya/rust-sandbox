use crate::entity::{Issue, IssueId};
use ulid::Ulid;

#[derive(Debug)]
pub struct CreateIssue {}

#[derive(Debug)]
pub struct IssueCreated {
    issue: Issue,
}

pub fn create_issue_workflow(_: CreateIssue) -> IssueCreated {
    let issue_id = IssueId::new(Ulid::new());
    let issue = Issue::new(issue_id);
    IssueCreated { issue }
}
