SELECT 
    sell_date,
    COUNT( DISTINCT product ) num_sold,
    GROUP_CONCAT(DISTINCT product ORDER BY product separator ',' ) products
FROM Activities 
GROUP BY sell_date 
ORDER BY 1;