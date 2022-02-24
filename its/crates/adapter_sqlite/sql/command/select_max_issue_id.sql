SELECT issue_ids.issue_number AS issue_number,
  issue_ids.aggregate_id AS aggregate_id
FROM issue_ids
WHERE issue_ids.issue_number = (
    SELECT MAX(issue_number)
    FROM issue_ids
  )
