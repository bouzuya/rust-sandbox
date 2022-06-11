SELECT id,
  resolution,
  status,
  title,
  due
FROM issues
WHERE id = ?
