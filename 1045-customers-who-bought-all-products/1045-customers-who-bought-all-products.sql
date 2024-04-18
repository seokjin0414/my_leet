WITH cnt AS (
SELECT COUNT(product_key) val FROM Product
)
SELECT customer_id 
FROM Customer 
GROUP BY customer_id
HAVING COUNT(DISTINCT product_key) = (SELECT val FROM cnt)