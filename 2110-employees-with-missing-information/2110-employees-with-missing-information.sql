SELECT e.employee_id
FROM Employees e
LEFT JOIN Salaries s USING(employee_id)
WHERE s.employee_id IS NULL
UNION
SELECT s.employee_id
FROM Employees e
RIGHT JOIN Salaries s USING(employee_id)
WHERE e.employee_id IS NULL
ORDER BY 1
