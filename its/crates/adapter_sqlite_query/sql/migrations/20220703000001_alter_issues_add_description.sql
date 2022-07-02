DELETE FROM last_event_id
WHERE TRUE;
--
DROP TABLE IF EXISTS issues;
--
CREATE TABLE IF NOT EXISTS issues (
  id TEXT NOT NULL,
  resolution TEXT,
  status TEXT NOT NULL,
  title TEXT NOT NULL,
  due INTEGER,
  description TEXT NOT NULL,
  CONSTRAINT issues_pk PRIMARY KEY (id)
);
