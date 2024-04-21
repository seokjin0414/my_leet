SELECT e.employee_id, e.name, COUNT(ee.employee_id) reports_count, ROUND(AVG(ee.age)) average_age
FROM Employees e
INNER JOIN Employees ee ON e.employee_id = ee.reports_to
GROUP BY e.employee_id, e.name
ORDER BY 1