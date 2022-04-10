SELECT issue_ids.issue_number AS issue_number,
  issue_ids.event_stream_id AS event_stream_id
FROM issue_ids
WHERE issue_ids.issue_number = (
    SELECT MAX(issue_number)
    FROM issue_ids
  )
