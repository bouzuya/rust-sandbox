-- This migration is used by iko.
CREATE TABLE IF NOT EXISTS tmp_events (
  seq INTEGER AUTO_INCREMENT PRIMARY KEY,
  id CHAR(26) NOT NULL,
  event_stream_id CHAR(26) NOT NULL,
  version BIGINT NOT NULL,
  data TEXT NOT NULL,
  CONSTRAINT events3_uk1 UNIQUE (id),
  CONSTRAINT events3_uk2 UNIQUE (event_stream_id, version),
  CONSTRAINT events3_fk1 FOREIGN KEY (event_stream_id) REFERENCES event_streams (id)
);
