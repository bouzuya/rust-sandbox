SELECT issue_block_link_ids.issue_block_link_id AS issue_block_link_id,
  issue_block_link_ids.aggregate_id AS aggregate_id
FROM issue_block_link_ids
WHERE issue_block_link_ids.issue_block_link_id = $1
