-- This migration is used by iko.
DELETE FROM events;
DROP TABLE events;
ALTER TABLE tmp_events
  RENAME TO events;
