SELECT MAX(indexings.at)
FROM indexings
  INNER JOIN successful_indexings ON successful_indexings.indexing_id = indexings.id
