SELECT e1.name
FROM Employee e1
JOIN (
    SELECT managerId id, COUNT(*) AS directReports
    FROM Employee
    GROUP BY managerId
    HAVING COUNT(*) >= 5
) e2 USING(id)