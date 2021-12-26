use ulid::Ulid;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct IssueId(Ulid);

impl IssueId {
    pub fn new(id: Ulid) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
struct Issue {
    id: IssueId,
}

impl Issue {
    pub fn new(id: IssueId) -> Self {
        Self { id }
    }
}

#[derive(Debug)]
struct CreateIssue {}

#[derive(Debug)]
struct IssueCreated {
    issue: Issue,
}

fn workflow1(_: CreateIssue) -> IssueCreated {
    let issue_id = IssueId::new(Ulid::new());
    let issue = Issue::new(issue_id);
    IssueCreated { issue }
}

fn main() {
    let command = CreateIssue {};
    let event = workflow1(command);
    println!("issue created : {:?}", event);
}
