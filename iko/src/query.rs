mod migration_status_row;

use sqlx::{any::AnyArguments, query::Query, Any, Transaction};

use crate::migration_status::MigrationStatus;

use self::migration_status_row::MigrationStatusRow;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("no rows to update")]
    NoRowsToUpdate,
}

pub async fn create_migration_status_table(transaction: &mut Transaction<'_, Any>) -> Result<()> {
    sqlx::query(include_str!("./sql/create_table.sql"))
        .execute(transaction)
        .await?;
    Ok(())
}

pub async fn insert_migration_status(transaction: &mut Transaction<'_, Any>) -> Result<()> {
    sqlx::query(include_str!("./sql/insert.sql"))
        .execute(transaction)
        .await?;
    Ok(())
}

pub async fn select_migration_status(
    transaction: &mut Transaction<'_, Any>,
) -> Result<MigrationStatus> {
    let row: MigrationStatusRow = sqlx::query_as(include_str!("./sql/select.sql"))
        .fetch_one(transaction)
        .await?;
    Ok(MigrationStatus::from(row))
}

pub async fn update_migration_status(
    transaction: &mut Transaction<'_, Any>,
    current: &MigrationStatus,
    updated: &MigrationStatus,
) -> Result<()> {
    let query: Query<Any, AnyArguments> = sqlx::query(include_str!("./sql/update.sql"))
        .bind(i64::from(updated.current_version()))
        .bind(updated.updated_version().map(i64::from))
        .bind(updated.value().to_string())
        .bind(i64::from(current.current_version()))
        .bind(current.value().to_string());
    let rows_affected = query.execute(transaction).await?.rows_affected();
    if rows_affected != 1 {
        return Err(Error::NoRowsToUpdate);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO
    }
}
