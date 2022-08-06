SELECT issue_comment_ids.issue_comment_id AS issue_comment_id,
  issue_comment_ids.event_stream_id AS event_stream_id
FROM issue_comment_ids
WHERE issue_comment_ids.issue_comment_id = $1
