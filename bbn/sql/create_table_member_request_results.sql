CREATE TABLE IF NOT EXISTS member_request_results (
  member_request_id INTEGER PRIMARY KEY,
  at INTEGER NOT NULL,
  member_response_id INTEGER,
  -- nullable
  FOREIGN KEY (member_request_id) REFERENCES member_requests (id) ON DELETE CASCADE,
  FOREIGN KEY (member_response_id) REFERENCES member_responses (id)
)
