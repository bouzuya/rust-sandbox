UPDATE migration_status
SET value = $1
WHERE current_version = $2
  AND value = $3
