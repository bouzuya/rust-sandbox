use crate::{migration_status_value::MigrationStatusValue, version::Version};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("already applied")]
    AlreadyApplied,
    #[error("already in progress")]
    AlreadyInProgress,
    #[error("not in progress")]
    NotInProgress,
}

#[derive(Debug, Eq, PartialEq)]
pub enum MigrationStatus {
    InProgress {
        current_version: Version,
        updated_version: Version,
    },
    Completed {
        current_version: Version,
    },
}

impl MigrationStatus {
    pub fn complete(&self) -> Result<MigrationStatus, Error> {
        match self {
            MigrationStatus::InProgress {
                updated_version, ..
            } => Ok(Self::Completed {
                current_version: *updated_version,
            }),
            MigrationStatus::Completed { .. } => Err(Error::NotInProgress),
        }
    }

    pub fn current_version(&self) -> Version {
        *match self {
            MigrationStatus::InProgress {
                current_version, ..
            } => current_version,
            MigrationStatus::Completed { current_version } => current_version,
        }
    }

    pub fn in_progress(&self, version: Version) -> Result<MigrationStatus, Error> {
        match self {
            MigrationStatus::InProgress { .. } => Err(Error::AlreadyInProgress),
            MigrationStatus::Completed { current_version } if current_version >= &version => {
                Err(Error::AlreadyApplied)
            }
            MigrationStatus::Completed { current_version } => Ok(Self::InProgress {
                current_version: *current_version,
                updated_version: version,
            }),
        }
    }

    pub fn updated_version(&self) -> Option<Version> {
        match self {
            MigrationStatus::InProgress {
                updated_version, ..
            } => Some(*updated_version),
            MigrationStatus::Completed { .. } => None,
        }
    }

    pub fn value(&self) -> MigrationStatusValue {
        match self {
            MigrationStatus::InProgress { .. } => MigrationStatusValue::InProgress,
            MigrationStatus::Completed { .. } => MigrationStatusValue::Completed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() -> anyhow::Result<()> {
        let initial = MigrationStatus::Completed {
            current_version: Version::from(0),
        };
        assert_eq!(initial.current_version(), Version::from(0));
        assert_eq!(initial.updated_version(), None);
        assert_eq!(initial.value(), MigrationStatusValue::Completed);

        let in_progress = initial.in_progress(Version::from(1))?;
        assert_eq!(in_progress.current_version(), Version::from(0));
        assert_eq!(in_progress.updated_version(), Some(Version::from(1)));
        assert_eq!(in_progress.value(), MigrationStatusValue::InProgress);

        let completed = in_progress.complete()?;
        assert_eq!(completed.current_version(), Version::from(1));
        assert_eq!(completed.updated_version(), None);
        assert_eq!(completed.value(), MigrationStatusValue::Completed);
        Ok(())
    }

    #[test]
    fn errors() -> anyhow::Result<()> {
        let initial = MigrationStatus::Completed {
            current_version: Version::from(0),
        };
        assert_eq!(
            initial.in_progress(Version::from(0)),
            Err(Error::AlreadyApplied)
        );

        let in_progress = initial.in_progress(Version::from(1))?;
        assert_eq!(
            in_progress.in_progress(Version::from(2)),
            Err(Error::AlreadyInProgress)
        );

        let completed = in_progress.complete()?;
        assert_eq!(completed.complete(), Err(Error::NotInProgress));

        Ok(())
    }
}
