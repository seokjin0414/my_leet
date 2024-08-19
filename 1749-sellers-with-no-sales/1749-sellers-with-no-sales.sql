WITH data AS(
SELECT seller_id
FROM Orders
WHERE YEAR(sale_date) = 2020
GROUP BY seller_id
)
SELECT seller_name
FROM Seller s
WHERE seller_id NOT IN (SELECT * FROM data)
ORDER BY 1