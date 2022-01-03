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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let ulid = Ulid::new();
        let number = IssueNumber::try_from(1_usize)?;
        let title = IssueTitle::from_str("title1")?;
        let issue = Issue::new(IssueId::new(ulid), number, title.clone());
        assert_eq!(issue.number(), number);
        assert_eq!(issue.title(), &title);
        Ok(())
    }
}
