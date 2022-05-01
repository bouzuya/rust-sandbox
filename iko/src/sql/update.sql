UPDATE migration_status
SET current_version = $1,
  updated_version = $2,
  value = $3
WHERE current_version = $4
  AND value = $5
