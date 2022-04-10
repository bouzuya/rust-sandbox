SELECT issue_block_link_ids.issue_block_link_id AS issue_block_link_id,
  issue_block_link_ids.event_stream_id AS event_stream_id
FROM issue_block_link_ids
WHERE issue_block_link_ids.issue_block_link_id = $1
