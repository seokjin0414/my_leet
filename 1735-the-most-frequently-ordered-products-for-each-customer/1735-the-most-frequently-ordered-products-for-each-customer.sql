WITH sorted AS (
    SELECT customer_id, product_id, RANK() OVER(PARTITION BY customer_id ORDER BY COUNT(*) DESC) num
    FROM Orders
    GROUP BY 1, 2
)
SELECT s.customer_id, s.product_id, p.product_name
FROM sorted s
INNER JOIN Products p USING(product_id)
WHERE s.num = 1