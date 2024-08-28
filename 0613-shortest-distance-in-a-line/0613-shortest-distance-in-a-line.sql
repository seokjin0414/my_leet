SELECT MIN(n - x) shortest
FROM
(SELECT x, LEAD(x) OVER(ORDER BY x) n FROM Point) p