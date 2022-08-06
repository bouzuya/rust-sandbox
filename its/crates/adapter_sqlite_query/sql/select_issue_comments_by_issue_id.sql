SELECT id,
  text,
  created_at,
  updated_at
FROM issue_comments
WHERE issue_id = ?
