SELECT c.customer_id, c.name
FROM Customers c
INNER JOIN Orders o ON c.customer_id = o.customer_id AND o.order_date BETWEEN "2020-06-01" AND "2020-07-30"
INNER JOIN Product p ON o.product_id = p.product_id
GROUP BY 1
HAVING SUM(CASE WHEN MONTH(o.order_date) = 06 THEN o.quantity * p.price END) >= 100
AND SUM(CASE WHEN MONTH(o.order_date) = 07 THEN o.quantity * p.price END) >= 100