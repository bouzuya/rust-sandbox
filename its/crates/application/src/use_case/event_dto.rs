use domain::{IssueAggregateEvent, IssueCreated, IssueFinished};
use serde::{Deserialize, Serialize};

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
            IssueAggregateEvent::Created(IssueCreated {
                at,
                issue_id,
                issue_title,
                version,
            }) => {
                EventDto::IssueCreated {
                    at: at.to_string(),
                    // TODO: IssueId::to_string
                    issue_id: usize::from(issue_id.issue_number()).to_string(),
                    issue_title: issue_title.to_string(),
                    version: u64::from(version),
                }
            }
            IssueAggregateEvent::Finished(IssueFinished {
                at,
                issue_id,
                version,
            }) => {
                EventDto::IssueFinished {
                    at: at.to_string(),
                    // TODO: IssueId::to_string
                    issue_id: usize::from(issue_id.issue_number()).to_string(),
                    version: u64::from(version),
                }
            }
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
    fn from_domain_issue_created_test() -> anyhow::Result<()> {
        let event = IssueAggregateEvent::Created(IssueCreated {
            at: Instant::from_str("2021-02-03T04:05:06Z")?,
            issue_id: IssueId::new(IssueNumber::try_from(2_usize)?),
            issue_title: IssueTitle::from_str("title1")?,
            version: Version::from(1_u64),
        });
        let dto = EventDto::IssueCreated {
            at: "2021-02-03T04:05:06Z".to_string(),
            issue_id: "2".to_string(),
            issue_title: "title1".to_string(),
            version: 1_u64,
        };
        let serialized = r#"{"type":"issue_created","at":"2021-02-03T04:05:06Z","issue_id":"2","issue_title":"title1","version":1}"#;
        assert_eq!(EventDto::from(event), dto);
        assert_eq!(serde_json::to_string(&dto)?, serialized);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized)?, dto);
        Ok(())
    }

    #[test]
    fn from_domain_issue_finished_test() -> anyhow::Result<()> {
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
        assert_eq!(EventDto::from(event), dto);
        assert_eq!(serde_json::to_string(&dto)?, serialized);
        assert_eq!(serde_json::from_str::<'_, EventDto>(serialized)?, dto);
        Ok(())
    }
}
