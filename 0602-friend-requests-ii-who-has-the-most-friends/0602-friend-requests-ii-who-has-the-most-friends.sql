SELECT id, COUNT(*) num
FROM (
    SELECT requester_id id FROM RequestAccepted
    UNION ALL
    SELECT accepter_id id FROM RequestAccepted
) all_ids
GROUP BY id
ORDER BY 2 DESC
LIMIT 1;