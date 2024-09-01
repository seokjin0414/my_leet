WITH min_log AS (
    SELECT player_id, MIN(event_date) event_date
    FROM Activity
    GROUP BY 1
)
SELECT player_id, device_id
FROM Activity
WHERE (player_id, event_date) IN (SELECT * FROM min_log)
