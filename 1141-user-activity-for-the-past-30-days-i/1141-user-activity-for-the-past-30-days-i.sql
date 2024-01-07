SELECT activity_date day, COUNT(DISTINCT user_id) active_users
FROM Activity
WHERE DATEDIFF('2019-07-27', activity_date) >= 0 AND DATEDIFF('2019-07-27', activity_date) < 30
GROUP BY 1
