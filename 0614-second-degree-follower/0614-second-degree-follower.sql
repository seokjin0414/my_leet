SELECT followee follower, COUNT(follower) num
FROM Follow
WHERE followee IN (SELECT followee FROM Follow)
AND followee IN (SELECT follower FROM Follow)
GROUP BY 1
ORDER BY 1