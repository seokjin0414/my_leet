WITH cal AS (
    SELECT 
        order_id, 
        MAX(quantity) max_v, 
        SUM(quantity) / COUNT(product_id) avg_v
    FROM OrdersDetails
    GROUP BY 1
)
SELECT order_id
FROM cal
WHERE max_v > (Select MAX(avg_v) FROM cal);