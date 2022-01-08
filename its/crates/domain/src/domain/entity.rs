use thiserror::Error;

use crate::{IssueId, IssueNumber, IssueStatus, IssueTitle};

#[derive(Debug, Error)]
pub enum IssueError {
    #[error("AlreadyFinished")]
    AlreadyFinished,
}

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

    pub fn finish(&self) -> Result<Self, IssueError> {
        if self.status == IssueStatus::Done {
            return Err(IssueError::AlreadyFinished);
        }
        Ok(Self {
            id: self.id.clone(),
            status: IssueStatus::Done,
            title: self.title.clone(),
        })
    }

    pub fn id(&self) -> &IssueId {
        &self.id
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
        let id = IssueId::new(number);
        let issue = Issue::new(id.clone(), title.clone());
        assert_eq!(issue.id(), &id);
        assert_eq!(issue.number(), number);
        assert_eq!(issue.status(), IssueStatus::Todo);
        assert_eq!(issue.title(), &title);
        Ok(())
    }

    #[test]
    fn finish_test() -> anyhow::Result<()> {
        let number = IssueNumber::try_from(1_usize)?;
        let title = IssueTitle::from_str("title1")?;
        let issue = Issue::new(IssueId::new(number), title);
        let updated = issue.finish()?;
        assert_eq!(updated.status(), IssueStatus::Done);
        Ok(())
    }
}
