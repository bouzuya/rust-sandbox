UPDATE database_version
SET migration_status = $1
WHERE current_version = $2
  AND migration_status = $3
