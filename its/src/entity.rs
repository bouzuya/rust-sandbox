mod issue_number;
mod issue_title;

use ulid::Ulid;

pub use self::issue_number::IssueNumber;
pub use self::issue_title::IssueTitle;

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
    title: IssueTitle,
}

impl Issue {
    pub fn new(id: IssueId, number: IssueNumber, title: IssueTitle) -> Self {
        Self { id, number, title }
    }

    pub fn number(&self) -> IssueNumber {
        self.number
    }

    pub fn title(&self) -> &IssueTitle {
        &self.title
    }
}

// TODO: tests
