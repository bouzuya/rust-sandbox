SELECT author_name,
  content,
  draft,
  edit_url,
  edited,
  entry_id,
  published,
  title,
  updated,
  url
FROM entries
WHERE updated BETWEEN ? AND ?
