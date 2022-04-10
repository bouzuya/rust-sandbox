-- CREATE TABLE tmp_issue_ids ...
CREATE TABLE IF NOT EXISTS tmp_issue_ids (
  issue_number INTEGER NOT NULL,
  event_stream_id CHAR(26) NOT NULL,
  CONSTRAINT issue_ids3_pk PRIMARY KEY (issue_number),
  CONSTRAINT issue_ids3_uk1 UNIQUE (event_stream_id),
  CONSTRAINT issue_ids3_fk1 FOREIGN KEY (event_stream_id) REFERENCES event_streams (id)
);
-- INSERT INTO tmp_issue_ids SELECT ... FROM issue_ids
INSERT INTO tmp_issue_ids (issue_number, event_stream_id)
SELECT issue_number,
  aggregate_id
FROM issue_ids;
-- DELETE FROM issue_ids
DELETE FROM issue_ids;
-- DROP TABLE issue_ids
DROP TABLE issue_ids;
-- ALTER TABLE tmp_issue_ids RENAME TO issue_ids
ALTER TABLE tmp_issue_ids
  RENAME TO issue_ids;
