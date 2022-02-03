CREATE TABLE IF NOT EXISTS issue_ids (
  issue_id TEXT NOT NULL,
  aggregate_id CHAR(26) NOT NULL,
  CONSTRAINT issue_ids_pk PRIMARY KEY (issue_id),
  CONSTRAINT issue_ids_uk UNIQUE (aggregate_id)
)
