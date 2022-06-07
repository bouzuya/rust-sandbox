SELECT events.id AS id,
  events.event_stream_id AS event_stream_id,
  events.version AS version,
  events.data AS data
FROM events
WHERE events.seq > (
    SELECT e2.seq
    FROM events e2
    WHERE e2.id = $1
  )
ORDER BY events.seq ASC
