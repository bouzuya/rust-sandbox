CREATE TABLE events (
  aggregate_id BLOB NOT NULL,
  version BIGINT NOT NULL,
  data BLOB NOT NULL,
  CONSTRAINT events_pk PRIMARY KEY (aggregate_id, version),
  CONSTRAINT events_fk1 FOREIGN KEY (aggregate_id) REFERENCES aggregates (id)
)
