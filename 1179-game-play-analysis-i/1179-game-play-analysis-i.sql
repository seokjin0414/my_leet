SELECT player_id, MIN(event_date) first_login
FROM Activity
GROUP BY 1