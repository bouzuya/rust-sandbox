SELECT entry_id,
  author_name,
  content,
  draft,
  edited,
  edit_url,
  published,
  title,
  updated,
  url
FROM entries
WHERE updated = ?
