SELECT w.name warehouse_name,
    SUM(w.units * p.Width * p.Length * p.Height) volume
FROM Warehouse w
INNER JOIN Products p USING(product_id)
GROUP BY 1