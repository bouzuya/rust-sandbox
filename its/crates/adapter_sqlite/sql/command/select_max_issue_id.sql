SELECT issue_ids.issue_id AS issue_id,
  issue_ids.aggregate_id AS aggregate_id
FROM issue_ids
WHERE issue_ids.issue_id = (
    SELECT MAX(issue_id)
    FROM issue_ids
  )
