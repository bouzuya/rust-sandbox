use std::future::Future;

use sqlx::AnyPool;

use crate::{migration::Migration, Error};

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
    pub(crate) fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.0.iter(),
        }
    }

    pub fn push<Fut>(
        &mut self,
        version: u32,
        migrate: impl Fn(AnyPool) -> Fut + 'static,
    ) -> Result<(), Error>
    where
        Fut: Future<Output = sqlx::Result<()>> + 'static,
    {
        if version == 0 {
            return Err(Error::ReservedVersion);
        }
        self.0.push(Migration::from((version, migrate)));
        Ok(())
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
