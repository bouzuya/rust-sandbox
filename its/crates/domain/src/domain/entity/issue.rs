use thiserror::Error;

use crate::{
    IssueCreatedV2, IssueDescription, IssueDue, IssueId, IssueNumber, IssueResolution, IssueStatus,
    IssueTitle,
};

#[derive(Debug, Error)]
pub enum IssueError {
    #[error("AlreadyFinished")]
    AlreadyFinished,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Issue {
    id: IssueId,
    resolution: Option<IssueResolution>,
    status: IssueStatus,
    title: IssueTitle,
    due: Option<IssueDue>,
    description: IssueDescription,
}

impl Issue {
    pub(crate) fn from_event(event: IssueCreatedV2) -> Self {
        Self {
            id: event.issue_id,
            resolution: None,
            status: IssueStatus::Todo,
            title: event.issue_title,
            due: event.issue_due,
            description: IssueDescription::default(),
        }
    }

    pub(crate) fn new(
        id: IssueId,
        title: IssueTitle,
        due: Option<IssueDue>,
        description: IssueDescription,
    ) -> Self {
        Self {
            id,
            resolution: None,
            status: IssueStatus::Todo,
            title,
            due,
            description,
        }
    }

    pub(crate) fn finish(&self, resolution: Option<IssueResolution>) -> Result<Self, IssueError> {
        if self.status == IssueStatus::Done {
            return Err(IssueError::AlreadyFinished);
        }
        Ok(Self {
            id: self.id.clone(),
            resolution,
            status: IssueStatus::Done,
            title: self.title.clone(),
            due: self.due,
            description: self.description.clone(),
        })
    }

    pub(crate) fn change_description(&self, description: IssueDescription) -> Self {
        Self {
            id: self.id.clone(),
            resolution: self.resolution.clone(),
            status: self.status,
            title: self.title.clone(),
            due: self.due(),
            description,
        }
    }

    pub(crate) fn change_due(&self, due: Option<IssueDue>) -> Self {
        Self {
            id: self.id.clone(),
            resolution: self.resolution.clone(),
            status: self.status,
            title: self.title.clone(),
            due,
            description: self.description.clone(),
        }
    }

    pub(crate) fn change_title(&self, title: IssueTitle) -> Self {
        Self {
            id: self.id.clone(),
            resolution: self.resolution.clone(),
            status: self.status,
            title,
            due: self.due(),
            description: self.description.clone(),
        }
    }

    pub(crate) fn id(&self) -> &IssueId {
        &self.id
    }

    pub(crate) fn number(&self) -> IssueNumber {
        self.id.issue_number()
    }

    pub(crate) fn resolution(&self) -> Option<&IssueResolution> {
        self.resolution.as_ref()
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

    pub(crate) fn description(&self) -> &IssueDescription {
        &self.description
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
        let description = IssueDescription::from_str("desc1")?;
        let issue = Issue::new(id.clone(), title.clone(), Some(due), description.clone());
        assert_eq!(issue.id(), &id);
        assert_eq!(issue.number(), number);
        assert_eq!(issue.status(), IssueStatus::Todo);
        assert_eq!(issue.title(), &title);
        assert_eq!(issue.due(), Some(due));
        assert_eq!(issue.description(), &description);
        Ok(())
    }

    #[test]
    fn change_description_test() -> anyhow::Result<()> {
        let issue = new()?;
        let description = IssueDescription::from_str("desc2")?;
        assert_eq!(
            issue.change_description(description.clone()).description(),
            &description
        );
        Ok(())
    }

    #[test]
    fn change_due_test() -> anyhow::Result<()> {
        let issue = new()?;
        assert_eq!(issue.change_due(None).due(), None);
        let due = IssueDue::from_str("1970-01-01T00:00:00Z")?;
        assert_eq!(issue.change_due(Some(due)).due(), Some(due));
        Ok(())
    }

    #[test]
    fn change_title_test() -> anyhow::Result<()> {
        let issue = new()?;
        let title = IssueTitle::from_str("title2")?;
        assert_eq!(issue.change_title(title.clone()).title(), &title);
        Ok(())
    }

    #[test]
    fn finish_test() -> anyhow::Result<()> {
        let number = IssueNumber::try_from(1_usize)?;
        let title = IssueTitle::from_str("title1")?;
        let description = IssueDescription::from_str("desc1")?;
        let issue = Issue::new(IssueId::new(number), title, None, description);
        let resolution = IssueResolution::from_str("Duplicate")?;
        let updated = issue.finish(Some(resolution.clone()))?;
        assert_eq!(updated.status(), IssueStatus::Done);
        assert_eq!(updated.resolution(), Some(&resolution));
        Ok(())
    }

    fn new() -> anyhow::Result<Issue> {
        let number = IssueNumber::try_from(1_usize)?;
        let title = IssueTitle::from_str("title1")?;
        let id = IssueId::new(number);
        let due = IssueDue::from_str("2021-02-03T04:05:06Z")?;
        let description = IssueDescription::from_str("desc1")?;
        Ok(Issue::new(id, title, Some(due), description))
    }
}
