INSERT INTO issue_block_links (
    issue_id,
    issue_title,
    blocked_issue_id,
    blocked_issue_title
  )
VALUES ($1, $2, $3, $4)
