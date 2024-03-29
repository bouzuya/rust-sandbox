use std::str::FromStr;

use crate::{
    aggregate::{
        issue::{
            attribute::{IssueDue, IssueResolution},
            IssueDescription, IssueTitle,
        },
        issue_comment::event::{
            issue_comment_created::IssueCommentCreatedJson,
            issue_comment_deleted::IssueCommentDeletedJson,
            issue_comment_updated::IssueCommentUpdatedJson, IssueCommentCreated,
            IssueCommentDeleted, IssueCommentUpdated,
        },
        IssueAggregateEvent, IssueBlockLinkAggregateEvent,
    },
    DomainEvent, IssueBlockLinkId, IssueBlocked, IssueCreatedV2, IssueDescriptionUpdated,
    IssueFinished, IssueId, IssueTitleUpdated, IssueUnblocked, IssueUpdated, Version,
};
use limited_date_time::{Instant, ParseInstantError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TryFromEventDtoError {
    #[error("Instant")]
    Instant(#[from] ParseInstantError),
    #[error("IssueDescription")]
    IssueDescription(#[from] crate::aggregate::issue::attribute::issue_description::Error),
    #[error("IssueDue")]
    IssueDue(#[from] crate::aggregate::issue::attribute::issue_due::Error),
    #[error("IssueId")]
    IssueId(#[from] crate::issue_id::ParseIssueIdError),
    #[error("IssueNumber")]
    IssueNumber(#[from] crate::issue_number::Error),
    #[error("IssueResolution")]
    IssueResolution(#[from] crate::aggregate::issue::attribute::issue_resolution::Error),
    #[error("IssueTitle")]
    IssueTitle(#[from] crate::aggregate::issue::attribute::issue_title::Error),
    #[error("IssueBlockLinkId")]
    IssueBlockLinkId(#[from] crate::issue_block_link_id::Error),
    #[error("NotIssueAggregate")]
    NotIssueAggregate,
    #[error("issue_comment_created error {0}")]
    IssueCommentCreated(
        #[from] crate::aggregate::issue_comment::event::issue_comment_created::Error,
    ),
    #[error("issue_comment_deleted error {0}")]
    IssueCommentDeleted(
        #[from] crate::aggregate::issue_comment::event::issue_comment_deleted::Error,
    ),
    #[error("issue_comment_updated error {0}")]
    IssueCommentUpdated(
        #[from] crate::aggregate::issue_comment::event::issue_comment_updated::Error,
    ),
}

#[derive(Clone, Debug, Eq, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
#[allow(clippy::enum_variant_names)]
pub enum EventDto {
    #[serde(rename = "issue_blocked")]
    IssueBlocked {
        at: String,
        issue_id: String,
        blocked_issue_id: String,
        version: u64,
    },
    #[serde(rename = "issue_comment_created")]
    IssueCommentCreated(IssueCommentCreatedJson),
    #[serde(rename = "issue_comment_deleted")]
    IssueCommentDeleted(IssueCommentDeletedJson),
    #[serde(rename = "issue_comment_updated")]
    IssueCommentUpdated(IssueCommentUpdatedJson),
    #[serde(rename = "issue_created")]
    IssueCreatedV1 {
        at: String,
        issue_id: String,
        issue_title: String,
        issue_due: Option<String>,
        issue_description: Option<String>,
        version: u64,
    },
    #[serde(rename = "issue_description_updated")]
    IssueDescriptionUpdated {
        at: String,
        issue_id: String,
        description: String,
        version: u64,
    },
    #[serde(rename = "issue_finished")]
    IssueFinished {
        at: String,
        issue_id: String,
        resolution: Option<String>,
        version: u64,
    },
    #[serde(rename = "issue_unblocked")]
    IssueUnblocked {
        at: String,
        issue_id: String,
        blocked_issue_id: String,
        version: u64,
    },
    #[serde(rename = "issue_title_updated")]
    IssueTitleUpdated {
        at: String,
        issue_id: String,
        issue_title: String,
        version: u64,
    },
    #[serde(rename = "issue_updated")]
    IssueUpdated {
        at: String,
        issue_id: String,
        issue_due: Option<String>,
        version: u64,
    },
}

impl From<DomainEvent> for EventDto {
    fn from(event: DomainEvent) -> Self {
        match event {
            DomainEvent::Issue(event) => EventDto::from(event),
            DomainEvent::IssueBlockLink(event) => EventDto::from(event),
            DomainEvent::IssueComment(event) => EventDto::from(event),
        }
    }
}

impl From<IssueAggregateEvent> for EventDto {
    fn from(event: IssueAggregateEvent) -> Self {
        use crate::aggregate::issue::IssueAggregateEvent::*;
        match event {
            Created(event) => EventDto::from(DomainEvent::from(IssueAggregateEvent::CreatedV2(
                IssueCreatedV2::from(event),
            ))),
            CreatedV2(event) => EventDto::IssueCreatedV1 {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                issue_title: event.issue_title().to_string(),
                issue_due: event.issue_due().map(|d| d.to_string()),
                issue_description: Some(event.issue_description.to_string()),
                version: u64::from(event.version()),
            },
            DescriptionUpdated(event) => EventDto::IssueDescriptionUpdated {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                description: event.issue_description().to_string(),
                version: u64::from(event.version()),
            },
            Finished(event) => EventDto::IssueFinished {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                resolution: event.resolution().map(|r| r.to_string()),
                version: u64::from(event.version()),
            },
            Updated(event) => EventDto::IssueUpdated {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                issue_due: event.issue_due().map(|d| d.to_string()),
                version: u64::from(event.version()),
            },
            TitleUpdated(event) => EventDto::IssueTitleUpdated {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                issue_title: event.issue_title().to_string(),
                version: u64::from(event.version()),
            },
        }
    }
}

impl From<IssueBlockLinkAggregateEvent> for EventDto {
    fn from(event: IssueBlockLinkAggregateEvent) -> Self {
        use crate::aggregate::issue_block_link::IssueBlockLinkAggregateEvent::*;
        match event {
            Blocked(event) => EventDto::IssueBlocked {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                blocked_issue_id: event.blocked_issue_id().to_string(),
                version: u64::from(event.version()),
            },
            Unblocked(event) => EventDto::IssueUnblocked {
                at: event.at().to_string(),
                issue_id: event.issue_id().to_string(),
                blocked_issue_id: event.blocked_issue_id().to_string(),
                version: u64::from(event.version()),
            },
        }
    }
}

impl From<crate::aggregate::issue_comment::Event> for EventDto {
    fn from(event: crate::aggregate::issue_comment::Event) -> Self {
        use crate::aggregate::issue_comment::Event::*;
        match event {
            Created(event) => EventDto::IssueCommentCreated(IssueCommentCreatedJson::from(event)),
            Deleted(event) => EventDto::IssueCommentDeleted(IssueCommentDeletedJson::from(event)),
            Updated(event) => EventDto::IssueCommentUpdated(IssueCommentUpdatedJson::from(event)),
        }
    }
}

impl TryFrom<EventDto> for DomainEvent {
    type Error = TryFromEventDtoError;

    fn try_from(value: EventDto) -> Result<Self, Self::Error> {
        match value {
            EventDto::IssueBlocked {
                at,
                issue_id,
                blocked_issue_id,
                version,
            } => Ok(
                IssueBlockLinkAggregateEvent::Blocked(IssueBlocked::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueBlockLinkId::new(
                        IssueId::from_str(issue_id.as_str())?,
                        IssueId::from_str(blocked_issue_id.as_str())?,
                    )?,
                    Version::from(version),
                ))
                .into(),
            ),
            EventDto::IssueCommentCreated(dto) => Ok(crate::aggregate::issue_comment::Event::from(
                IssueCommentCreated::try_from(dto)?,
            )
            .into()),
            EventDto::IssueCommentDeleted(dto) => Ok(crate::aggregate::issue_comment::Event::from(
                IssueCommentDeleted::try_from(dto)?,
            )
            .into()),
            EventDto::IssueCommentUpdated(dto) => Ok(crate::aggregate::issue_comment::Event::from(
                IssueCommentUpdated::try_from(dto)?,
            )
            .into()),
            EventDto::IssueCreatedV1 {
                at,
                issue_id,
                issue_title,
                issue_due,
                issue_description,
                version,
            } => Ok(
                IssueAggregateEvent::CreatedV2(IssueCreatedV2::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueId::from_str(issue_id.as_str())?,
                    IssueTitle::try_from(issue_title)?,
                    issue_due
                        .map(|s| IssueDue::from_str(s.as_str()))
                        .transpose()?,
                    issue_description
                        .map(|s| IssueDescription::from_str(s.as_str()))
                        .transpose()?
                        .unwrap_or_default(),
                    Version::from(version),
                ))
                .into(),
            ),
            EventDto::IssueDescriptionUpdated {
                at,
                issue_id,
                description,
                version,
            } => Ok(IssueAggregateEvent::DescriptionUpdated(
                IssueDescriptionUpdated::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueId::from_str(issue_id.as_str())?,
                    IssueDescription::from_str(description.as_str())?,
                    Version::from(version),
                ),
            )
            .into()),
            EventDto::IssueFinished {
                at,
                issue_id,
                resolution,
                version,
            } => Ok(
                IssueAggregateEvent::Finished(IssueFinished::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueId::from_str(issue_id.as_str())?,
                    resolution
                        .as_ref()
                        .map(|s| IssueResolution::from_str(s))
                        .transpose()?,
                    Version::from(version),
                ))
                .into(),
            ),
            EventDto::IssueUnblocked {
                at,
                issue_id,
                blocked_issue_id,
                version,
            } => Ok(
                IssueBlockLinkAggregateEvent::Unblocked(IssueUnblocked::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueBlockLinkId::new(
                        IssueId::from_str(issue_id.as_str())?,
                        IssueId::from_str(blocked_issue_id.as_str())?,
                    )?,
                    Version::from(version),
                ))
                .into(),
            ),
            EventDto::IssueTitleUpdated {
                at,
                issue_id,
                issue_title,
                version,
            } => Ok(
                IssueAggregateEvent::TitleUpdated(IssueTitleUpdated::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueId::from_str(issue_id.as_str())?,
                    IssueTitle::from_str(issue_title.as_str())?,
                    Version::from(version),
                ))
                .into(),
            ),
            EventDto::IssueUpdated {
                at,
                issue_id,
                issue_due,
                version,
            } => Ok(
                IssueAggregateEvent::Updated(IssueUpdated::from_trusted_data(
                    Instant::from_str(at.as_str())?,
                    IssueId::from_str(issue_id.as_str())?,
                    issue_due
                        .map(|s| IssueDue::from_str(s.as_str()))
                        .transpose()?,
                    Version::from(version),
                ))
                .into(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{IssueCreated, IssueId, IssueNumber, Version};
    use limited_date_time::Instant;

    use super::*;

    #[test]
    fn issue_created_dto_v1_and_issue_created_serialized_v1_0_conversion_test() -> anyhow::Result<()>
    {
        let dto = EventDto::IssueCreatedV1 {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_title: "title1".to_string(),
            issue_due: None,
            issue_description: None,
            version: 1_u64,
        };
        let serialized_v1_0 = r#"{"type":"issue_created","at":"2021-02-03T04:05:06Z","issue_id":"2","issue_title":"title1","version":1}"#;
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized_v1_0)?, dto);
        // serialization removed
        Ok(())
    }

    #[test]
    fn issue_created_dto_v1_and_issue_created_serialized_v1_1_conversion_test() -> anyhow::Result<()>
    {
        let dto = EventDto::IssueCreatedV1 {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_title: "title1".to_string(),
            issue_due: None,
            issue_description: None,
            version: 1_u64,
        };
        let serialized_v1_1 = r#"{"type":"issue_created","at":"2021-02-03T04:05:06Z","issue_id":"2","issue_title":"title1","issue_due":null,"version":1}"#;
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized_v1_1)?, dto);
        // serialization removed
        Ok(())
    }

    #[test]
    fn issue_created_dto_v1_and_issue_created_serialized_v1_2_conversion_test() -> anyhow::Result<()>
    {
        let dto = EventDto::IssueCreatedV1 {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_title: "title1".to_string(),
            issue_due: None,
            issue_description: Some("desc1".to_string()),
            version: 1_u64,
        };
        let serialized_v1_2 = r#"{"type":"issue_created","at":"2021-02-03T04:05:06Z","issue_id":"2","issue_title":"title1","issue_due":null,"issue_description":"desc1","version":1}"#;
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized_v1_2)?, dto);
        assert_eq!(serde_json::to_string(&dto)?, serialized_v1_2);
        Ok(())
    }

    #[test]
    fn issue_created_dto_v1_and_issue_created_event_conversion_test() -> anyhow::Result<()> {
        let event = DomainEvent::from(IssueAggregateEvent::Created(
            IssueCreated::from_trusted_data(
                Instant::from_str("2021-02-03T04:05:06Z")?,
                IssueId::new(IssueNumber::try_from(2_usize)?),
                IssueTitle::from_str("title1")?,
                Version::from(1_u64),
            ),
        ));
        let dto = EventDto::IssueCreatedV1 {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_title: "title1".to_string(),
            issue_due: None,
            // DomainEvent::from で CreatedV2 を経由するため None ではなく Some("") になる
            // これで良いのか怪しい
            issue_description: Some("".to_string()),
            version: 1_u64,
        };
        assert_eq!(EventDto::from(event), dto);
        // serialization removed
        Ok(())
    }

    #[test]
    fn issue_created_event_v2_and_issue_created_dto_v1_conversion_test() -> anyhow::Result<()> {
        let event = DomainEvent::from(IssueAggregateEvent::CreatedV2(
            IssueCreatedV2::from_trusted_data(
                Instant::from_str("2021-02-03T04:05:06Z")?,
                IssueId::new(IssueNumber::try_from(2_usize)?),
                IssueTitle::from_str("title1")?,
                Some(IssueDue::from_str("2021-02-03T04:05:07Z")?),
                IssueDescription::from_str("desc1")?,
                Version::from(1_u64),
            ),
        ));
        let dto = EventDto::IssueCreatedV1 {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_title: "title1".to_string(),
            issue_due: Some("2021-02-03T04:05:07Z".to_string()),
            issue_description: Some("desc1".to_string()),
            version: 1_u64,
        };
        assert_eq!(EventDto::from(event.clone()), dto);
        assert_eq!(DomainEvent::try_from(dto)?, event);

        let event = DomainEvent::from(IssueAggregateEvent::CreatedV2(
            IssueCreatedV2::from_trusted_data(
                Instant::from_str("2021-02-03T04:05:06Z")?,
                IssueId::new(IssueNumber::try_from(2_usize)?),
                IssueTitle::from_str("title1")?,
                Some(IssueDue::from_str("2021-02-03T04:05:07Z")?),
                IssueDescription::default(),
                Version::from(1_u64),
            ),
        ));
        let dto = EventDto::IssueCreatedV1 {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_title: "title1".to_string(),
            issue_due: Some("2021-02-03T04:05:07Z".to_string()),
            issue_description: Some("".to_string()),
            version: 1_u64,
        };
        assert_eq!(EventDto::from(event.clone()), dto);
        assert_eq!(DomainEvent::try_from(dto)?, event);
        Ok(())
    }

    #[test]
    fn issue_finished_v1_0_conversion_test() -> anyhow::Result<()> {
        // v1.0 deserialize only
        let event = DomainEvent::from(IssueAggregateEvent::Finished(
            IssueFinished::from_trusted_data(
                Instant::from_str("2021-02-03T04:05:06Z")?,
                IssueId::new(IssueNumber::try_from(2_usize)?),
                None,
                Version::from(1_u64),
            ),
        ));
        let dto = EventDto::IssueFinished {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            resolution: None,
            version: 1_u64,
        };
        let serialized =
            r#"{"type":"issue_finished","at":"2021-02-03T04:05:06Z","issue_id":"2","version":1}"#;
        assert_eq!(EventDto::from(event.clone()), dto);
        assert_eq!(DomainEvent::try_from(EventDto::from(event.clone()))?, event);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized)?, dto);
        Ok(())
    }

    #[test]
    fn issue_finished_v1_1_conversion_test() -> anyhow::Result<()> {
        // v1.1
        let event = DomainEvent::from(IssueAggregateEvent::Finished(
            IssueFinished::from_trusted_data(
                Instant::from_str("2021-02-03T04:05:06Z")?,
                IssueId::new(IssueNumber::try_from(2_usize)?),
                Some(IssueResolution::from_str("Duplicate")?),
                Version::from(1_u64),
            ),
        ));
        let dto = EventDto::IssueFinished {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            resolution: Some("Duplicate".to_string()),
            version: 1_u64,
        };
        let serialized = r#"{"type":"issue_finished","at":"2021-02-03T04:05:06Z","issue_id":"2","resolution":"Duplicate","version":1}"#;
        assert_eq!(EventDto::from(event.clone()), dto);
        assert_eq!(DomainEvent::try_from(EventDto::from(event.clone()))?, event);
        assert_eq!(serde_json::to_string(&dto)?, serialized);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized)?, dto);

        // v1.1
        let event = DomainEvent::from(IssueAggregateEvent::Finished(
            IssueFinished::from_trusted_data(
                Instant::from_str("2021-02-03T04:05:06Z")?,
                IssueId::new(IssueNumber::try_from(2_usize)?),
                None,
                Version::from(1_u64),
            ),
        ));
        let dto = EventDto::IssueFinished {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            resolution: None,
            version: 1_u64,
        };
        let serialized = r#"{"type":"issue_finished","at":"2021-02-03T04:05:06Z","issue_id":"2","resolution":null,"version":1}"#;
        assert_eq!(EventDto::from(event.clone()), dto);
        assert_eq!(DomainEvent::try_from(EventDto::from(event.clone()))?, event);
        assert_eq!(serde_json::to_string(&dto)?, serialized);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized)?, dto);

        Ok(())
    }
    #[test]
    fn issue_updated_conversion_test() -> anyhow::Result<()> {
        let event = DomainEvent::from(IssueAggregateEvent::Updated(
            IssueUpdated::from_trusted_data(
                Instant::from_str("2021-02-03T04:05:06Z")?,
                IssueId::new(IssueNumber::try_from(2_usize)?),
                Some(IssueDue::from_str("2021-02-03T04:05:07Z")?),
                Version::from(1_u64),
            ),
        ));
        let dto = EventDto::IssueUpdated {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_due: Some("2021-02-03T04:05:07Z".to_string()),
            version: 1_u64,
        };
        let serialized = r#"{"type":"issue_updated","at":"2021-02-03T04:05:06Z","issue_id":"2","issue_due":"2021-02-03T04:05:07Z","version":1}"#;
        assert_eq!(EventDto::from(event.clone()), dto);
        assert_eq!(DomainEvent::try_from(EventDto::from(event.clone()))?, event);
        assert_eq!(serde_json::to_string(&dto)?, serialized);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized)?, dto);
        Ok(())
    }
}
