WITH data AS (
    SELECT seat_id 
    FROM Cinema 
    WHERE free   
)
SELECT *
FROM data
WHERE seat_id +1 IN (SELECT * FROM data)
OR seat_id - 1 IN (SELECT * FROM data);