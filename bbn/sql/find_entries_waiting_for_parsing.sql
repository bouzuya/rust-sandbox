SELECT member_responses.body
FROM member_responses
WHERE ? IS NULL
  OR member_responses.at > ?
ORDER BY member_responses.at ASC
