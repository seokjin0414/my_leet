SELECT DISTINCT Num ConsecutiveNums
FROM Logs
WHERE (Id + 1, Num) IN (SELECT * FROM Logs)
    AND (Id + 2, Num) IN (SELECT * FROM Logs)