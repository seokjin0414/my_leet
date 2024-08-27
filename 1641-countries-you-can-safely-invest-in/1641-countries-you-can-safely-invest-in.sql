WITH avg AS (
    SELECT AVG(duration) FROM Calls
)
SELECT c.name country
FROM Country c
INNER JOIN Person p ON LEFT(p.phone_number, 3) = c.country_code
INNER JOIN Calls ca ON ca.caller_id = p.id OR ca.callee_id = p.id
GROUP BY 1
HAVING AVG(duration) > (SELECT * FROM avg)