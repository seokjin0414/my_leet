WITH sorted AS (
    SELECT order_id, order_date, customer_id, RANK() OVER(PARTITION BY customer_id ORDER BY order_date DESC) num
    FROM Orders
)
SELECT c.name customer_name, s.customer_id, s.order_id, s.order_date
FROM sorted s
INNER JOIN Customers c ON s.customer_id = c.customer_id
WHERE s.num <= 3
ORDER BY 1, 2, 4 DESC