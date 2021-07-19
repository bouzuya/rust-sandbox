SELECT collection_responses.body
FROM indexing_collection_responses
  INNER JOIN collection_responses ON collection_responses.id = indexing_collection_responses.collection_response_id
WHERE indexing_id = ?
ORDER BY collection_responses.id ASC
