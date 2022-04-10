-- CREATE TABLE event_streams ...
CREATE TABLE IF NOT EXISTS event_streams (
  id CHAR(26) NOT NULL,
  version INTEGER NOT NULL DEFAULT 0,
  CONSTRAINT event_streams_pk PRIMARY KEY (id)
);
-- INSERT INTO event_streams SELECT ... FROM aggregates
INSERT INTO event_streams (id, version)
SELECT id,
  version
FROM aggregates;
