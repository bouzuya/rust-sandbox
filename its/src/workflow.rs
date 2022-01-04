use crate::entity::{Issue, IssueId, IssueNumber, IssueTitle};
use limited_date_time::Instant;

#[derive(Debug)]
pub struct CreateIssue {
    issue_title: IssueTitle,
}

impl CreateIssue {
    pub fn new(issue_title: IssueTitle) -> Self {
        Self { issue_title }
    }
}

#[derive(Debug)]
pub struct IssueCreated {
    at: Instant,
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

pub fn create_issue_workflow(create_issue: CreateIssue) -> IssueCreated {
    let issue_repository = IssueRepository::default(); // TODO: dependency

    let issue_number = issue_repository.next_issue_number();
    let issue_id = IssueId::new(issue_number);
    let issue = Issue::new(issue_id, create_issue.issue_title);
    let at = Instant::now();
    IssueCreated { at, issue }
}
