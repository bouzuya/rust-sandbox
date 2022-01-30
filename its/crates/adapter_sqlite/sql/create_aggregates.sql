CREATE TABLE aggregates (
  id CHAR(26) NOT NULL,
  version INTEGER NOT NULL DEFAULT 0,
  CONSTRAINT aggregates_pk PRIMARY KEY (id)
)
