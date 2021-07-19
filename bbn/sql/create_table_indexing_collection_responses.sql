CREATE TABLE IF NOT EXISTS indexing_collection_responses (
  indexing_id INTEGER NOT NULL,
  collection_response_id INTEGER NOT NULL,
  PRIMARY KEY (indexing_id, collection_response_id),
  FOREIGN KEY (indexing_id) REFERENCES indexings (id) ON DELETE CASCADE,
  FOREIGN KEY (collection_response_id) REFERENCES collection_responses (id) ON DELETE CASCADE
)
