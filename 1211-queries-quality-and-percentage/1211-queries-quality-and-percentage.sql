SELECT query_name, ROUND(AVG(rating / position), 2) quality, ROUND(AVG(IF(rating < 3, 1, 0))*100, 2) poor_query_percentage
FROM Queries
WHERE query_name IS NOT NULL
GROUP BY 1