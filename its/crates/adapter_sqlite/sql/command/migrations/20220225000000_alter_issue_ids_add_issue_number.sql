CREATE TABLE IF NOT EXISTS issue_ids2 (
  issue_number INTEGER NOT NULL,
  aggregate_id CHAR(26) NOT NULL,
  CONSTRAINT issue_ids_pk2 PRIMARY KEY (issue_number),
  CONSTRAINT issue_ids_uk2 UNIQUE (aggregate_id),
  CONSTRAINT issue_ids_fk2 FOREIGN KEY (aggregate_id) REFERENCES aggregates (id)
);
INSERT INTO issue_ids2 (issue_number, aggregate_id)
SELECT CAST(issue_id AS INTEGER),
  aggregate_id
FROM issue_ids;
DELETE FROM issue_ids;
DROP TABLE issue_ids;
ALTER TABLE issue_ids2
  RENAME TO issue_ids;
