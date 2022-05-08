use std::future::Future;

use sqlx::AnyPool;

use crate::{migration::Migration, Error};

#[derive(Default)]
pub struct Migrations(pub(crate) Vec<Migration>);

impl Migrations {
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
    #[test]
    fn test() {
        // TODO
    }
}
