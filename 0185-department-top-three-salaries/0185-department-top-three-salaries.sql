SELECT d.name Department, e.name Employee, e.salary Salary
FROM Employee e
INNER JOIN Department d ON e.departmentId = d.id
WHERE 
(
    SELECT COUNT(DISTINCT salary)
    FROM Employee e2
    WHERE e2.departmentId = e.departmentId 
    AND e2.salary >= e.salary
) <= 3
ORDER BY 1, 3 DESC;