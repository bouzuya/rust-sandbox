use std::str::FromStr;

use domain::{
    aggregate::IssueAggregateEvent, IssueCreatedV2, IssueDue, IssueFinished, IssueId, IssueTitle,
    ParseIssueDueError, ParseIssueIdError, ParseIssueNumberError, TryFromIssueTitleError, Version,
};
use limited_date_time::{Instant, ParseInstantError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TryFromEventDtoError {
    #[error("Instant")]
    Instant(#[from] ParseInstantError),
    #[error("IssueDue")]
    IssueDue(#[from] ParseIssueDueError),
    #[error("IssueId")]
    IssueId(#[from] ParseIssueIdError),
    #[error("IssueNumber")]
    IssueNumber(#[from] ParseIssueNumberError),
    #[error("IssueTitle")]
    IssueTitle(#[from] TryFromIssueTitleError),
}

#[derive(Debug, Eq, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum EventDto {
    #[serde(rename = "issue_created")]
    IssueCreated {
        at: String,
        issue_id: String,
        issue_title: String,
        issue_due: Option<String>,
        version: u64,
    },
    #[serde(rename = "issue_finished")]
    IssueFinished {
        at: String,
        issue_id: String,
        version: u64,
    },
}

// TODO: DomainEvent -> EventDto
impl From<IssueAggregateEvent> for EventDto {
    fn from(event: IssueAggregateEvent) -> Self {
        match event {
            IssueAggregateEvent::Created(event) => EventDto::from(IssueAggregateEvent::CreatedV2(
                IssueCreatedV2::from_v1(event),
            )),
            IssueAggregateEvent::CreatedV2(event) => EventDto::IssueCreated {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                issue_title: event.issue_title().to_string(),
                issue_due: event.issue_due().map(|d| d.to_string()),
                version: u64::from(event.version()),
            },
            IssueAggregateEvent::Finished(event) => EventDto::IssueFinished {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                version: u64::from(event.version()),
            },
        }
    }
}

// TODO: EventDto -> DomainEvent
impl TryFrom<EventDto> for IssueAggregateEvent {
    type Error = TryFromEventDtoError;

    fn try_from(value: EventDto) -> Result<Self, Self::Error> {
        match value {
            EventDto::IssueCreated {
                at,
                issue_id,
                issue_title,
                issue_due,
                version,
            } => Ok(IssueAggregateEvent::CreatedV2(
                IssueCreatedV2::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueId::from_str(issue_id.as_str())?,
                    IssueTitle::try_from(issue_title)?,
                    issue_due
                        .map(|s| IssueDue::from_str(s.as_str()))
                        .transpose()?,
                    Version::from(version),
                ),
            )),
            EventDto::IssueFinished {
                at,
                issue_id,
                version,
            } => Ok(IssueAggregateEvent::Finished(
                IssueFinished::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueId::from_str(issue_id.as_str())?,
                    Version::from(version),
                ),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use domain::{IssueCreated, IssueId, IssueNumber, IssueTitle, Version};
    use limited_date_time::Instant;

    use super::*;

    #[test]
    fn issue_created_conversion_test() -> anyhow::Result<()> {
        let event = IssueAggregateEvent::Created(IssueCreated::from_trusted_data(
            Instant::from_str("2021-02-03T04:05:06Z")?,
            IssueId::new(IssueNumber::try_from(2_usize)?),
            IssueTitle::from_str("title1")?,
            Version::from(1_u64),
        ));
        let dto = EventDto::IssueCreated {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_title: "title1".to_string(),
            issue_due: None,
            version: 1_u64,
        };
        let serialized_v1_1 = r#"{"type":"issue_created","at":"2021-02-03T04:05:06Z","issue_id":"2","issue_title":"title1","issue_due":null,"version":1}"#;
        assert_eq!(EventDto::from(event), dto);
        assert_eq!(serde_json::to_string(&dto)?, serialized_v1_1);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized_v1_1)?, dto);
        // dto -> event (IssueCreatedV1 -> IssueCreatedV2)

        let serialized_v1_0 = r#"{"type":"issue_created","at":"2021-02-03T04:05:06Z","issue_id":"2","issue_title":"title1","version":1}"#;
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized_v1_0)?, dto);
        Ok(())
    }

    #[test]
    fn issue_created_v2_conversion_test() -> anyhow::Result<()> {
        let event = IssueAggregateEvent::CreatedV2(IssueCreatedV2::from_trusted_data(
            Instant::from_str("2021-02-03T04:05:06Z")?,
            IssueId::new(IssueNumber::try_from(2_usize)?),
            IssueTitle::from_str("title1")?,
            Some(IssueDue::from_str("2021-02-03T04:05:07Z")?),
            Version::from(1_u64),
        ));
        let dto = EventDto::IssueCreated {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_title: "title1".to_string(),
            issue_due: Some("2021-02-03T04:05:07Z".to_string()),
            version: 1_u64,
        };
        let serialized_v1_1 = r#"{"type":"issue_created","at":"2021-02-03T04:05:06Z","issue_id":"2","issue_title":"title1","issue_due":"2021-02-03T04:05:07Z","version":1}"#;
        assert_eq!(EventDto::from(event.clone()), dto);
        assert_eq!(
            IssueAggregateEvent::try_from(EventDto::from(event.clone()))?,
            event
        );
        assert_eq!(serde_json::to_string(&dto)?, serialized_v1_1);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized_v1_1)?, dto);
        Ok(())
    }

    #[test]
    fn issue_finished_conversion_test() -> anyhow::Result<()> {
        let event = IssueAggregateEvent::Finished(IssueFinished::from_trusted_data(
            Instant::from_str("2021-02-03T04:05:06Z")?,
            IssueId::new(IssueNumber::try_from(2_usize)?),
            Version::from(1_u64),
        ));
        let dto = EventDto::IssueFinished {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            version: 1_u64,
        };
        let serialized =
            r#"{"type":"issue_finished","at":"2021-02-03T04:05:06Z","issue_id":"2","version":1}"#;
        assert_eq!(EventDto::from(event.clone()), dto);
        assert_eq!(
            IssueAggregateEvent::try_from(EventDto::from(event.clone()))?,
            event
        );
        assert_eq!(serde_json::to_string(&dto)?, serialized);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized)?, dto);
        Ok(())
    }
}
