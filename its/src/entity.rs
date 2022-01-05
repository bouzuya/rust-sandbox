mod issue_id;
mod issue_number;
mod issue_status;
mod issue_title;

pub use self::issue_id::IssueId;
pub use self::issue_number::IssueNumber;
use self::issue_status::IssueStatus;
pub use self::issue_title::IssueTitle;

#[derive(Clone, Debug)]
pub struct Issue {
    id: IssueId,
    status: IssueStatus,
    title: IssueTitle,
}

impl Issue {
    pub fn new(id: IssueId, title: IssueTitle) -> Self {
        Self {
            id,
            status: IssueStatus::Todo,
            title,
        }
    }

    pub fn number(&self) -> IssueNumber {
        self.id.issue_number()
    }

    pub fn status(&self) -> IssueStatus {
        self.status
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
        let number = IssueNumber::try_from(1_usize)?;
        let title = IssueTitle::from_str("title1")?;
        let issue = Issue::new(IssueId::new(number), title.clone());
        assert_eq!(issue.number(), number);
        assert_eq!(issue.status(), IssueStatus::Todo);
        assert_eq!(issue.title(), &title);
        Ok(())
    }
}
