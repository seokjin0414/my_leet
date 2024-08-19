WITH sorted AS (
    SELECT o.sales_id
    FROM Orders o
    INNER JOIN Company c ON o.com_id = c.com_id AND c.name = "RED"
    GROUP BY 1
)
SELECT name
FROM SalesPerson
WHERE sales_id NOT IN(SELECT * FROM sorted)