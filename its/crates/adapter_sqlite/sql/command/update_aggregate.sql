UPDATE aggregates
SET version = $1
WHERE id = $2
  AND version = $3
