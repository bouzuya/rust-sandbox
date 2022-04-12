use thiserror::Error;

use crate::{IssueCreatedV2, IssueDue, IssueId, IssueNumber, IssueStatus, IssueTitle};

#[derive(Debug, Error)]
pub enum IssueError {
    #[error("AlreadyFinished")]
    AlreadyFinished,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Issue {
    id: IssueId,
    status: IssueStatus,
    title: IssueTitle,
    due: Option<IssueDue>,
}

impl Issue {
    pub(crate) fn from_event(event: IssueCreatedV2) -> Self {
        Self {
            id: event.issue_id,
            status: IssueStatus::Todo,
            title: event.issue_title,
            due: event.issue_due,
        }
    }

    pub(crate) fn new(id: IssueId, title: IssueTitle, due: Option<IssueDue>) -> Self {
        Self {
            id,
            status: IssueStatus::Todo,
            title,
            due,
        }
    }

    pub(crate) fn finish(&self) -> Result<Self, IssueError> {
        if self.status == IssueStatus::Done {
            return Err(IssueError::AlreadyFinished);
        }
        Ok(Self {
            id: self.id.clone(),
            status: IssueStatus::Done,
            title: self.title.clone(),
            due: self.due,
        })
    }

    pub(crate) fn change_due(&self, due: Option<IssueDue>) -> Self {
        Self {
            id: self.id.clone(),
            status: self.status,
            title: self.title.clone(),
            due,
        }
    }

    pub(crate) fn id(&self) -> &IssueId {
        &self.id
    }

    pub(crate) fn number(&self) -> IssueNumber {
        self.id.issue_number()
    }

    pub(crate) fn status(&self) -> IssueStatus {
        self.status
    }

    pub(crate) fn title(&self) -> &IssueTitle {
        &self.title
    }

    pub(crate) fn due(&self) -> Option<IssueDue> {
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
    fn change_due() -> anyhow::Result<()> {
        let issue = new()?;
        assert_eq!(issue.change_due(None).due(), None);
        let due = IssueDue::from_str("1970-01-01T00:00:00Z")?;
        assert_eq!(issue.change_due(Some(due)).due(), Some(due));
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

    fn new() -> anyhow::Result<Issue> {
        let number = IssueNumber::try_from(1_usize)?;
        let title = IssueTitle::from_str("title1")?;
        let id = IssueId::new(number);
        let due = IssueDue::from_str("2021-02-03T04:05:06Z")?;
        Ok(Issue::new(id, title, Some(due)))
    }
}
