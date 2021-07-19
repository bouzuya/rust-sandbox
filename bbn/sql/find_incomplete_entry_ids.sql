SELECT member_requests.entry_id
FROM member_requests
  LEFT OUTER JOIN member_request_results ON member_request_results.member_request_id = member_requests.id
WHERE member_request_results.member_request_id IS NULL
ORDER BY member_requests.at ASC
