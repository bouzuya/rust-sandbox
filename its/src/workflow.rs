use crate::entity::{Issue, IssueId, IssueNumber};
use ulid::Ulid;

#[derive(Debug)]
pub struct CreateIssue {}

#[derive(Debug)]
pub struct IssueCreated {
    issue: Issue,
}

#[derive(Debug, Default)]
pub struct IssueRepository {
    issues: Vec<Issue>,
}

impl IssueRepository {
    pub fn next_issue_number(&self) -> IssueNumber {
        if let Some(last_issue) = self.issues.last() {
            last_issue.number().next_number()
        } else {
            IssueNumber::start_number()
        }
    }
}

pub fn create_issue_workflow(_: CreateIssue) -> IssueCreated {
    let issue_repository = IssueRepository::default(); // TODO: dependency

    let issue_number = issue_repository.next_issue_number();
    let issue_id = IssueId::new(Ulid::new());
    let issue = Issue::new(issue_id, issue_number);
    IssueCreated { issue }
}
