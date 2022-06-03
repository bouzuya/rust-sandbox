SELECT events.id AS id,
  events.event_stream_id AS event_stream_id,
  events.version AS version,
  events.data AS data
FROM events
WHERE events.event_stream_id = $1
