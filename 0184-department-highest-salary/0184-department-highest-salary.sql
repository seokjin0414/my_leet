WITH top_salary AS (
    SELECT departmentId, MAX(salary) salary
    FROM Employee
    GROUP BY 1
)
SELECT d.name Department, e.name Employee, e.salary Salary
FROM Employee e
INNER JOIN top_salary t ON e.departmentId = t.departmentId AND e.salary = t.salary
INNER JOIN Department d ON e.departmentId = d.id