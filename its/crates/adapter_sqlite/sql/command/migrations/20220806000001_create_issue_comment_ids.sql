-- This migration is used by iko.
CREATE TABLE IF NOT EXISTS issue_comment_ids (
  issue_comment_id TEXT NOT NULL,
  event_stream_id CHAR(26) NOT NULL,
  CONSTRAINT issue_comment_ids_pk PRIMARY KEY (issue_comment_id),
  CONSTRAINT issue_comment_ids_uk UNIQUE (event_stream_id),
  CONSTRAINT issue_comment_ids_fk1 FOREIGN KEY (event_stream_id) REFERENCES event_streams (id)
)
