SELECT product_name, product_id, order_id, order_date
FROM (
    SELECT p.product_name, o.product_id, order_id, o.order_date, RANK() OVER(PARTITION BY p.product_name, p.product_id ORDER BY o.order_date DESC) as num
FROM Orders o
INNER JOIN Products p ON o.product_id = p.product_id) sorted
WHERE num = 1
ORDER BY 1, 2, 3