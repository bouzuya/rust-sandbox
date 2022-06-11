DELETE FROM issue_block_links
WHERE issue_id = $1
  AND blocked_issue_id = $2
