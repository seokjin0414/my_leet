SELECT e1.employee_id
FROM Employees e1
INNER JOIN Employees e2 ON e1.manager_id = e2.employee_id
INNER JOIN Employees e3 ON e2.manager_id = e3.employee_id
WHERE e1.employee_id != 1
AND (e2.manager_id = 1 or e3.manager_id = 1)