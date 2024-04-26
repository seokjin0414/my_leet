WITH all_logs AS (
    SELECT * FROM Logs
)
SELECT DISTINCT Num ConsecutiveNums
FROM Logs
WHERE (Id + 1, Num) IN (SELECT * FROM all_logs)
    AND (Id + 2, Num) IN (SELECT * FROM all_logs)