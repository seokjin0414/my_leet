WITH data AS (
    SELECT log_id, RANK() OVER(ORDER BY log_id) num
    FROM Logs
)
SELECT MIN(log_id) start_id, 
       MAX(log_id) end_id
FROM data
GROUP BY log_id - num