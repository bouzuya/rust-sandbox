UPDATE migration_status
SET current_version = $1,
  value = $2
WHERE current_version = $3
  AND value = $4
