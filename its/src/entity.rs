mod issue_number;

use ulid::Ulid;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct IssueId(Ulid);

impl IssueId {
    pub fn new(id: Ulid) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub struct Issue {
    id: IssueId,
}

impl Issue {
    pub fn new(id: IssueId) -> Self {
        Self { id }
    }
}
