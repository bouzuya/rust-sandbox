-- This migration is used by iko.
SELECT event_stream_id,
  version,
  data
FROM events
ORDER BY event_stream_id,
  version ASC;
