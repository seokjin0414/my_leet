WITH data AS (
    SELECT seat_id
    FROM Cinema
    WHERE free
) 
SELECT d1.seat_id 
FROM data d1
LEFT JOIN data d2 ON d1.seat_id = d2.seat_id - 1
LEFT JOIN data d3 ON d1.seat_id = d3.seat_id + 1
WHERE d2.seat_id IS NOT NULL 
OR d3.seat_id IS NOT NULL