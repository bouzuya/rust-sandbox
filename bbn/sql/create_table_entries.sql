CREATE TABLE IF NOT EXISTS entries (
  entry_id TEXT PRIMARY KEY,
  author_name TEXT NOT NULL,
  content TEXT NOT NULL,
  draft INTEGER NOT NULL,
  edited INTEGER NOT NULL,
  published INTEGER NOT NULL,
  title TEXT NOT NULL,
  updated INTEGER NOT NULL,
  parsed_at INTEGER NOT NULL
)
