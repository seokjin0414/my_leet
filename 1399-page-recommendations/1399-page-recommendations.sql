WITH friends AS (
    SELECT CASE WHEN user1_id = 1 THEN user2_id ELSE user1_id END user_id
    FROM Friendship
    WHERE user1_id = 1 OR user2_id = 1
),
liked AS (
    SELECT page_id
    FROM Likes
    WHERE user_id = 1

)
SELECT page_id recommended_page
FROM Likes l
INNER JOIN friends f USING (user_id)
LEFT JOIN liked d USING (page_id)
WHERE d.page_id IS NULL
GROUP BY 1