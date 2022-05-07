CREATE TABLE IF NOT EXISTS migration_status (
  current_version INTEGER PRIMARY KEY,
  updated_version INTEGER,
  value VARCHAR(11) NOT NULL
)
