WITH data AS (
SELECT customer_id, GROUP_CONCAT(product_name ORDER BY product_name SEPARATOR "") name_set
FROM Orders
GROUP BY 1
HAVING name_set LIKE "%AB%" AND name_set NOT LIKE "%C%"
)
SELECT c.*
FROM data d
INNER JOIN Customers c ON d.customer_id = c.customer_id
ORDER BY 1