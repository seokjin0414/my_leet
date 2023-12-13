SELECT a.Id
FROM Weather a, Weather b
WHERE a.Temperature > b.Temperature
AND DATEDIFF(a.RecordDate, b.RecordDate) = 1;