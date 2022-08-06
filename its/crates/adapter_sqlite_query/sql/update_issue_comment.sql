UPDATE issue_comments
SET text = $1,
  updated_at = $2
WHERE id = $3;
