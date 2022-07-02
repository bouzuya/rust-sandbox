SELECT id,
  resolution,
  status,
  title,
  due,
  description
FROM issues
WHERE id = ?
