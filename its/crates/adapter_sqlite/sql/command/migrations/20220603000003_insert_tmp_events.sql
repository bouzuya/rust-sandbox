-- This migration is used by iko.
INSERT INTO tmp_events (id, event_stream_id, version, data)
VALUES (?, ?, ?, ?);
