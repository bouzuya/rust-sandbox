SELECT entry_id,
  author_name,
  content,
  draft,
  edited,
  published,
  title,
  updated
FROM entries
WHERE updated = ?
