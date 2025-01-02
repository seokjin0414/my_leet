WITH wee AS (
    SELECT followee, COUNT(*) num
    FROM Follow
    GROUP BY 1
),
wer AS (
    SELECT follower
    FROM Follow
    GROUP BY follower
)
SELECT e.followee follower, num
FROM wee e
INNER JOIN wer r ON e.followee = r.follower
ORDER BY 1;