CREATE TABLE IF NOT EXISTS issue_block_links (
  issue_id TEXT NOT NULL,
  issue_title TEXT NOT NULL,
  blocked_issue_id TEXT NOT NULL,
  blocked_issue_title TEXT NOT NULL,
  CONSTRAINT issue_block_links_pk PRIMARY KEY (issue_id, blocked_issue_id)
);
CREATE INDEX IF NOT EXISTS issue_block_links_issue_id_index ON issue_block_links(issue_id);
CREATE INDEX IF NOT EXISTS issue_block_links_blocked_issue_id_index ON issue_block_links(blocked_issue_id);
