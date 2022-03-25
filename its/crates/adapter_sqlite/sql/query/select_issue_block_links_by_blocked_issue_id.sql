SELECT issue_id,
  issue_title,
  blocked_issue_id,
  blocked_issue_title
FROM issue_block_links
WHERE blocked_issue_id = ?
