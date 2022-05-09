use std::future::Future;

use sqlx::AnyPool;

use crate::{migration::Migration, migration_status::Version, Error};

pub struct Iter<'a> {
    inner: std::slice::Iter<'a, Migration>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Migration;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Default)]
pub struct Migrations(Vec<Migration>);

impl Migrations {
    pub(crate) fn iter(&self) -> Iter {
        Iter {
            inner: self.0.iter(),
        }
    }

    pub fn push<F, Fut>(&mut self, version: u32, migrate: F) -> Result<(), Error>
    where
        F: Fn(AnyPool) -> Fut + 'static,
        Fut: Future<Output = sqlx::Result<()>> + 'static,
    {
        if version == 0 {
            return Err(Error::ReservedVersion);
        }
        if Version::from(version) <= self.last_migration_version() {
            return Err(Error::IncorrectVersionOrder);
        }
        self.0.push(Migration::from((version, migrate)));
        Ok(())
    }

    fn last_migration_version(&self) -> Version {
        self.0
            .last()
            .map(Migration::version)
            .unwrap_or_else(|| Version::from(0))
    }
}

#[cfg(test)]
mod tests {
    use crate::migration_status::Version;

    use super::*;
    use sqlx::AnyPool;

    #[test]
    fn test() -> anyhow::Result<()> {
        async fn migrate1(_pool: AnyPool) -> sqlx::Result<()> {
            Ok(())
        }
        let mut migrations = Migrations::default();
        migrations.push(1, migrate1)?;
        let mut iter = migrations.iter();
        assert_eq!(Some(Version::from(1)), iter.next().map(|m| m.version()));
        assert_eq!(None, iter.next().map(|m| m.version()));
        Ok(())
    }
}
