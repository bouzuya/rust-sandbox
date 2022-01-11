use thiserror::Error;

use crate::{IssueDue, IssueId, IssueNumber, IssueStatus, IssueTitle};

use super::event::IssueCreated;

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
    due: Option<IssueDue>,
}

impl Issue {
    pub fn from_event(event: IssueCreated) -> Self {
        Self {
            id: event.issue_id,
            status: IssueStatus::Todo,
            title: event.issue_title,
            due: None, // TODO:
        }
    }

    pub fn new(id: IssueId, title: IssueTitle, due: Option<IssueDue>) -> Self {
        Self {
            id,
            status: IssueStatus::Todo,
            title,
            due,
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
            due: None, // TODO:
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

    pub fn due(&self) -> Option<IssueDue> {
        self.due
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
        let due = IssueDue::from_str("2021-02-03T04:05:06Z")?;
        let issue = Issue::new(id.clone(), title.clone(), Some(due));
        assert_eq!(issue.id(), &id);
        assert_eq!(issue.number(), number);
        assert_eq!(issue.status(), IssueStatus::Todo);
        assert_eq!(issue.title(), &title);
        assert_eq!(issue.due(), Some(due));
        Ok(())
    }

    #[test]
    fn finish_test() -> anyhow::Result<()> {
        let number = IssueNumber::try_from(1_usize)?;
        let title = IssueTitle::from_str("title1")?;
        let issue = Issue::new(IssueId::new(number), title, None);
        let updated = issue.finish()?;
        assert_eq!(updated.status(), IssueStatus::Done);
        Ok(())
    }
}
