CREATE TABLE IF NOT EXISTS successful_indexings (
  indexing_id INTEGER PRIMARY KEY,
  at INTEGER NOT NULL,
  FOREIGN KEY (indexing_id) REFERENCES indexings (id) ON DELETE CASCADE
)
