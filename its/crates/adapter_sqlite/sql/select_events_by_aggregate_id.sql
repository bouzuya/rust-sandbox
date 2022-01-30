SELECT events.aggregate_id AS aggregate_id,
  events.version AS version,
  events.data AS data
FROM events
WHERE events.aggregate_id = $1
