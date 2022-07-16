use limited_date_time::Instant;

use crate::{
    aggregate::issue::{
        attribute::{IssueDue, IssueTitle},
        IssueDescription,
    },
    IssueCreated, IssueId, Version,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCreatedV2 {
    pub(crate) at: Instant,
    pub(crate) issue_id: IssueId,
    pub(crate) issue_title: IssueTitle,
    pub(crate) issue_due: Option<IssueDue>,
    pub(crate) issue_description: IssueDescription,
    pub(crate) version: Version,
}

impl From<IssueCreated> for IssueCreatedV2 {
    fn from(event: IssueCreated) -> Self {
        Self::new(
            event.at,
            event.issue_id,
            event.issue_title,
            None,
            IssueDescription::default(),
            event.version,
        )
    }
}

impl IssueCreatedV2 {
    pub(crate) fn from_trusted_data(
        at: Instant,
        issue_id: IssueId,
        issue_title: IssueTitle,
        issue_due: Option<IssueDue>,
        issue_description: IssueDescription,
        version: Version,
    ) -> Self {
        Self::new(
            at,
            issue_id,
            issue_title,
            issue_due,
            issue_description,
            version,
        )
    }

    pub(crate) fn new(
        at: Instant,
        issue_id: IssueId,
        issue_title: IssueTitle,
        issue_due: Option<IssueDue>,
        issue_description: IssueDescription,
        version: Version,
    ) -> Self {
        Self {
            at,
            issue_id,
            issue_title,
            issue_due,
            issue_description,
            version,
        }
    }

    pub fn at(&self) -> Instant {
        self.at
    }

    pub fn issue_id(&self) -> &IssueId {
        &self.issue_id
    }

    pub fn issue_title(&self) -> &IssueTitle {
        &self.issue_title
    }

    pub fn issue_due(&self) -> Option<IssueDue> {
        self.issue_due
    }

    pub fn issue_description(&self) -> &IssueDescription {
        &self.issue_description
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

// TODO: tests
