CREATE TABLE last_event_id (
  event_id CHAR(26) NOT NULL,
  CONSTRAINT latest_event_id_pk PRIMARY KEY (event_id)
);
