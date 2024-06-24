WITH 
day_sum AS (
    SELECT visited_on, SUM(amount) amount
    FROM Customer
    GROUP BY visited_on
),
day7_sum AS (
    SELECT visited_on, SUM(amount) OVER (ORDER BY visited_on ROWS BETWEEN 6 PRECEDING AND CURRENT ROW) amount
    FROM day_sum
)
SELECT visited_on, amount, ROUND(amount / 7, 2) average_amount
FROM day7_sum
WHERE visited_on > (SELECT MIN(visited_on) FROM day_sum) + INTERVAL 5 DAY