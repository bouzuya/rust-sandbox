-- CREATE TABLE tmp_events ...
CREATE TABLE IF NOT EXISTS tmp_events (
  event_stream_id CHAR(26) NOT NULL,
  version BIGINT NOT NULL,
  data TEXT NOT NULL,
  CONSTRAINT events2_pk PRIMARY KEY (event_stream_id, version),
  CONSTRAINT events2_fk1 FOREIGN KEY (event_stream_id) REFERENCES event_streams (id)
);
-- INSERT INTO tmp_events SELECT ... FROM events
INSERT INTO tmp_events (event_stream_id, version, data)
SELECT aggregate_id,
  version,
  data
FROM events;
-- DELETE FROM events
DELETE FROM events;
-- DROP TABLE events
DROP TABLE events;
-- ALTER TABLE tmp_events RENAME TO events
ALTER TABLE tmp_events
  RENAME TO events;
