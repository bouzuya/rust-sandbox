UPDATE last_event_id
SET event_id = $1
WHERE event_id = $2;
