WITH data AS(
SELECT fail_date date, "failed" period_state
FROM failed
UNION ALL 
SELECT success_date date, "succeeded" period_state
FROM succeeded
),
sorted AS (    
SELECT date, period_state, ROW_NUMBER() OVER(ORDER BY period_state, date) num
FROM data 
WHERE date >= "2019-01-01" 
AND date <= "2019-12-31"
)
SELECT period_state, MIN(date) start_date, MAX(date) end_date 
FROM sorted
GROUP BY DATE_SUB(date, INTERVAL num DAY ), 1
ORDER BY 2
