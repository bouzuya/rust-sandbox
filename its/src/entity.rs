mod issue_number;

use ulid::Ulid;

pub use self::issue_number::IssueNumber;

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
    number: IssueNumber,
}

impl Issue {
    pub fn new(id: IssueId, number: IssueNumber) -> Self {
        Self { id, number }
    }

    pub fn number(&self) -> IssueNumber {
        self.number
    }
}
