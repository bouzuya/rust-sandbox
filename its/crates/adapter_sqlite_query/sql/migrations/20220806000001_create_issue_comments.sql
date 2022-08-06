CREATE TABLE IF NOT EXISTS issue_comments (
  id TEXT NOT NULL,
  issue_id TEXT NOT NULL,
  text TEXT NOT NULL,
  created_at TEXT NOT NULL,
  -- NULLable
  updated_at TEXT,
  CONSTRAINT issue_comments_pk PRIMARY KEY (id)
);
