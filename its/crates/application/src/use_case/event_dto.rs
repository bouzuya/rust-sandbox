use std::str::FromStr;

use domain::{
    IssueAggregateEvent, IssueCreated, IssueFinished, IssueId, IssueTitle, ParseIssueIdError,
    ParseIssueNumberError, TryFromIssueTitleError, Version,
};
use limited_date_time::{Instant, ParseInstantError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TryFromEventDtoError {
    #[error("Instant")]
    Instant(#[from] ParseInstantError),
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
            IssueAggregateEvent::Created(event) => EventDto::IssueCreated {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                issue_title: event.issue_title().to_string(),
                version: u64::from(event.version()),
            },
            IssueAggregateEvent::Finished(IssueFinished {
                at,
                issue_id,
                version,
            }) => EventDto::IssueFinished {
                at: at.to_string(),
                issue_id: issue_id.to_string(),
                version: u64::from(version),
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
                version,
            } => Ok(IssueAggregateEvent::Created(
                IssueCreated::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueId::from_str(issue_id.as_str())?,
                    IssueTitle::try_from(issue_title)?,
                    Version::from(version),
                ),
            )),
            EventDto::IssueFinished {
                at,
                issue_id,
                version,
            } => Ok(IssueAggregateEvent::Finished(IssueFinished {
                at: Instant::from_str(at.as_str())?,
                issue_id: IssueId::from_str(issue_id.as_str())?,
                version: Version::from(version),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use domain::{IssueId, IssueNumber, IssueTitle, Version};
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
            version: 1_u64,
        };
        let serialized = r#"{"type":"issue_created","at":"2021-02-03T04:05:06Z","issue_id":"2","issue_title":"title1","version":1}"#;
        assert_eq!(EventDto::from(event.clone()), dto);
        assert_eq!(
            IssueAggregateEvent::try_from(EventDto::from(event.clone()))?,
            event
        );
        assert_eq!(serde_json::to_string(&dto)?, serialized);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized)?, dto);
        Ok(())
    }

    #[test]
    fn issue_finished_conversion_test() -> anyhow::Result<()> {
        let event = IssueAggregateEvent::Finished(IssueFinished {
            at: Instant::from_str("2021-02-03T04:05:06Z")?,
            issue_id: IssueId::new(IssueNumber::try_from(2_usize)?),
            version: Version::from(1_u64),
        });
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
