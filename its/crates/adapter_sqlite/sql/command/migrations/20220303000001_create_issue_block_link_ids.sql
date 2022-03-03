CREATE TABLE IF NOT EXISTS issue_block_link_ids (
  issue_block_link_id TEXT NOT NULL,
  aggregate_id CHAR(26) NOT NULL,
  CONSTRAINT issue_block_link_ids_pk PRIMARY KEY (issue_id),
  CONSTRAINT issue_block_link_ids_uk UNIQUE (aggregate_id),
  CONSTRAINT issue_block_link_ids_fk1 FOREIGN KEY (aggregate_id) REFERENCES aggregates (id)
)
