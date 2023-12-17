SELECT e1.name
FROM Employee e1
INNER JOIN Employee e2 ON e2.managerId = e1.id
GROUP BY e1.id
HAVING COUNT(e1.id) >= 5 