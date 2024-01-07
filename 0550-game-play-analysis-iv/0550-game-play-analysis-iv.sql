WITH ac AS (SELECT player_id, MIN(event_date) min_event_date
            FROM Activity 
            GROUP BY player_id)
SELECT ROUND(COUNT(a.player_id) / (SELECT COUNT(player_id) FROM ac), 2) fraction
FROM Activity a 
INNER JOIN ac ON ac.player_id = a.player_id 
AND ac.min_event_date = DATE_SUB(a.event_date, INTERVAL 1 DAY)
 
