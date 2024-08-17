SELECT employee_id, CASE WHEN name LIKE("M%") OR employee_id % 2 = 0 THEN 0 ELSE salary END bonus
FROM Employees
ORDER BY 1