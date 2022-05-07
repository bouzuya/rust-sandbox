INSERT INTO migration_status(
    current_version,
    updated_version,
    value
  )
SELECT 0,
  NULL,
  'completed'
WHERE NOT EXISTS (
    SELECT current_version
    FROM migration_status
  )
