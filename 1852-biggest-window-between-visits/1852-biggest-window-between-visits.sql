WITH ranked AS (
    SELECT user_id,
    ABS(DATEDIFF(visit_date, IFNULL(LEAD(visit_date) OVER(PARTITION BY user_id ORDER BY visit_date), '2021-1-1'))) value
    FROM UserVisits 
)
SELECT user_id, MAX(value) biggest_window
FROM ranked    
GROUP BY 1
ORDER BY 1
