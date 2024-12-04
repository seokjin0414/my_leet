WITH winner AS (
    SELECT candidateId 
    FROM Vote 
    GROUP BY candidateId 
    ORDER BY COUNT(*) DESC LIMIT 1
)
SELECT name
FROM Candidate
WHERE id = (SELECT * FROM winner)