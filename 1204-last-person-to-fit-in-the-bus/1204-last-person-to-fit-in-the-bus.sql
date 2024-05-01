SELECT person_name 
FROM
(SELECT person_name, weight, turn, SUM(weight) OVER(ORDER BY turn) s
FROM queue) a
WHERE s <= 1000
ORDER BY turn DESC
LIMIT 1;