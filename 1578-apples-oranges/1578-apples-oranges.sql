SELECT sale_date, 
    SUM(CASE WHEN fruit = "apples" THEN sold_num ELSE -sold_num END) diff
FROM Sales
GROUP BY 1