UPDATE database_version
SET current_version = $1,
  migration_status = $2
WHERE current_version = $3
  AND migration_status = $4
