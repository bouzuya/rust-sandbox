-- CREATE TABLE tmp_issue_block_link_ids ...
CREATE TABLE IF NOT EXISTS tmp_issue_block_link_ids (
  issue_block_link_id TEXT NOT NULL,
  event_stream_id CHAR(26) NOT NULL,
  CONSTRAINT issue_block_link_ids2_pk PRIMARY KEY (issue_block_link_id),
  CONSTRAINT issue_block_link_ids2_uk1 UNIQUE (event_stream_id),
  CONSTRAINT issue_block_link_ids2_fk1 FOREIGN KEY (event_stream_id) REFERENCES event_streams (id)
);
-- INSERT INTO tmp_issue_block_link_ids SELECT ... FROM issue_block_link_ids
INSERT INTO tmp_issue_block_link_ids (issue_block_link_id, event_stream_id)
SELECT issue_block_link_id,
  aggregate_id
FROM issue_block_link_ids;
-- DELETE FROM issue_block_link_ids
DELETE FROM issue_block_link_ids;
-- DROP TABLE issue_block_link_ids
DROP TABLE issue_block_link_ids;
-- ALTER TABLE tmp_issue_block_link_ids RENAME TO issue_block_link_ids
ALTER TABLE tmp_issue_block_link_ids
  RENAME TO issue_block_link_ids;
